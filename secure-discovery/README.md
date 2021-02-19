# Secure Discovery
​This is the code that corresponds to the privacy-conscious patient discovery tool, which uses and extends the core functionalities of the [MedCo framework](https://medco.epfl.ch). More information about the MedCo framework and codebase, and detailed steps for its deployment can be found at the [MedCo documentation](https://medco.epfl.ch/documentation/).


## Repository Organization
### Deployment Profiles
This subfolder contains deployment profiles, the used keys, configuration and docker-compose files needed to run the experiments in a network configured with different numbers of nodes and database sizes.
Note that these deployments contain configuration specific to a certain set of EPFL machines, which need to be modified in order to be reproduced in other machines.
In order to make a test deployment for yourself, you can follow the later instructions.

### Experiments
This subfolder contains the code needed to replicate an existing i2b2 database and extend it to reach the scales tested in the article (up to 28 billion observations), the SQL scripts to insert these data in a running database (deployed with the aforementioned deployment profiles), the scripts to run the experiments, and the logs for the obtained results.
​
### Crypto Operations
This subfolder contains the code and instructions needed to reproduce the experiments for data loading timings, including the local encryption time for an increasing number of variables, and the equality-preserving re-encryption time before an individual record is inserted in the database.


## MedCo Test Deployment
By following these instructions, you will be able to deploy a test deployment of three MedCo nodes on your Linux `localhost` machine (for other scenarios refer to the technical documentation, MacOS deployment can work with more RAM allocated to the Docker VM, and Windows deployment is untested).
Note that this test deployment will contain only a small amount of demo data, as the actual data used for the experiments are massive (see the end note about it). 
Those instructions are a targetted and summarized version of the MedCo technical documentation, please refer to it for more complete instructions.

You will need Docker and Docker-Compose installed on your machine to pursue this test.

### Deploy MedCo Nodes
Execute the following set of commands to download the 0.2.1-1 release of MedCo and deploy it:

```bash
cd ~
wget https://github.com/ldsec/medco-deployment/archive/v0.2.1-1.tar.gz
tar xvzf v0.2.1-1.tar.gz
mv medco-deployment-0.2.1-1 medco-deployment
cd medco-deployment/compose-profiles/test-local-3nodes
docker-compose pull
docker-compose up -d
```

Note that the first deployment initializes the database and that can take up to 10 minutes.
Check the i2b2 logs with `docker-compose logs i2b2-srv0` to inspect the status of the initialization.


### Configure Keycloak
Configure Keycloak by accessing its administration interface at http://localhost/auth/admin and using the credentials `keycloak`/`keycloak` and follow the following the steps.


#### Import default settings
Import the provided realm configuration into Keycloak. This will create the MedCo client with the appropriate roles.

- Go to the Import menu
- Click on Select file and select the file `keycloak-medco-realm.json` that you will find in `~/medco-deployment/resources/configuration`.
- Select to import everything, and to Skip if resources already exist


#### Configure the MedCo OpenID Connect client
- Go to the Clients menu and click on the `medco` client.
- In the Settings tab, replace the existing Valid Redirect URIs with the value `http://localhost/glowing-bear`.
- In the same tab, fill Web Origins with `+` and save.


#### Add a user and attribute permissions
- Go to the Users menu, click on Add user.
- Fill the Username field, toggle to ON the Email Verified button and click Save.
- Go to the Credentials tab, enter twice the user’s password, toggle to OFF the Temporary button if desired and click Reset Password.
- Go to the Role Mappings tab, and select medco in the Client Roles.
- Add all the roles.


### Load MedCo Demo Data
First download the data with the provided helper script:

```bash
cd ~/medco-deployment/resources/data
bash download.sh
```

Then execute the following commands to load data in the 3 nodes and restart the deployment.

```bash
cd ~/medco-deployment/compose-profiles/test-local-3nodes
docker-compose -f docker-compose.tools.yml run medco-loader-srv0 v0 \
    --ont_clinical /data/genomic/tcga_cbio/8_clinical_data.csv \
    --sen /data/genomic/sensitive.txt \
    --ont_genomic /data/genomic/tcga_cbio/8_mutation_data.csv \
    --clinical /data/genomic/tcga_cbio/8_clinical_data.csv \
    --genomic /data/genomic/tcga_cbio/8_mutation_data.csv \
    --output /data/
docker-compose -f docker-compose.tools.yml run medco-loader-srv1 v0 \
    --ont_clinical /data/genomic/tcga_cbio/8_clinical_data.csv \
    --sen /data/genomic/sensitive.txt \
    --ont_genomic /data/genomic/tcga_cbio/8_mutation_data.csv \
    --clinical /data/genomic/tcga_cbio/8_clinical_data.csv \
    --genomic /data/genomic/tcga_cbio/8_mutation_data.csv \
    --output /data/
docker-compose -f docker-compose.tools.yml run medco-loader-srv2 v0 \
    --ont_clinical /data/genomic/tcga_cbio/8_clinical_data.csv \
    --sen /data/genomic/sensitive.txt \
    --ont_genomic /data/genomic/tcga_cbio/8_mutation_data.csv \
    --clinical /data/genomic/tcga_cbio/8_clinical_data.csv \
    --genomic /data/genomic/tcga_cbio/8_mutation_data.csv \
    --output /data/
docker-compose stop
docker-compose up -d
```

At this stage the MedCo deployment should be up and running. Access `http://localhost/glowing-bear` to test it with the previously configured credentials.



## Data Generation
The data used for some of the experiments are massive, in the order of tens of Terabytes, thus they are not directly shareable.
As they are generated from an original real dataset, you will find here the instructions how to generate them.

### Original Data
The original data was obtained by combining several TCGA datasets available from [cBioPortal](https://www.cbioportal.org/).
This combined dataset is hosted at [this address](https://github.com/ldsec/projects-data/tree/master/medco/datasets/genomic/tcga_cbio) and is comprised of two files:
- clinical_data.csv
- mutation_data.csv

Note that when you execute as previously explained the download script `download.sh`, this data will be included.
You will need to first perform the normal MedCo data loading with this dataset, i.e. use the previously shown commands for data loading and replace:
-  `8_clinical_data.csv` by `clinical_data.csv` 
-  `8_mutation_data.csv` by `mutation_data.csv` 

### Data Replication
Once you have the original dataset loaded in MedCo, you are ready to proceed with the replication.
For this [some general information are available in the MedCo documentation](https://ldsec.gitbook.io/medco-documentation/developers/database), and here are the key steps:

- To accomodate those large data, change some database settings and table definitions with the following:
```sql
-- structure
ALTER TABLE i2b2demodata_i2b2.observation_fact
    ALTER COLUMN instance_num TYPE bigint,
    ALTER COLUMN text_search TYPE bigint;
    
-- settings
ALTER SYSTEM SET maintenance_work_mem TO '32GB';
SELECT pg_reload_conf();
```

- Run the duplication (with the method "2") with the following:
```sql
SELECT i2b2demodata_i2b2.obs_fact_duplication_method_2(1212);
```
This will give you a database replicated 1212 times, that includes approximately 50k patients and 9.5B records per node.
Then simply copy this database over 2 other nodes to reach **150k patients and 28.5B records over 3 nodes**.

### Data Reduction
In order to distribute this data over more nodes (6, 9 and 12) while keeping the same total amount of data, this database needs to be reducted.
- For 6 nodes, reduce to 4.75B records:
```sql
SELECT i2b2demodata_i2b2.obs_fact_reduction(4750000000);
```
- For 9 nodes, reduce to 3.15B records:
```sql
SELECT i2b2demodata_i2b2.obs_fact_reduction(3170000000);
```
- For 12 nodes, reduce to 2.37B records:
```sql
SELECT i2b2demodata_i2b2.obs_fact_reduction(2370000000);
```

Once reducted on one node, simply copy the database on the other nodes.

### Regenerating Indexes
After generating each version of the database, it is very important to re-generate the indexes of the i2b2 database.
Due to the method used for the duplication and reduction, the indexes will not be kept up to date!
Please also note that this step will be very long (e.g. up to 90 hours on a very powerful machine).
The command to run is the following:
```sql
SELECT i2b2demodata_i2b2.obs_fact_indexes();
```
