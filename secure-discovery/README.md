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
This subfolder contains the code needed to reproduce the experiments for data loading timings, including the local encryption time for an increasing number of variables, and the equality-preserving re-encryption time before an individual record is inserted in the database.


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
Check the i2b2 logs with `docker-compose logs test-local-3nodes_i2b2-srv0_1` to inspect the status of the initialization.


### Configure Keycloak
Configure Keycloak by accessing its administration interface at http://localhost/auth/admin and using the credentials `keycloak`/`keycloak` and follow the following the steps.


#### Import default settings
Import the provided realm configuration into Keycloak. This will create the MedCo client with the appropriate roles.

- Go to the Import menu
- Click on Select file and select the file `keycloak-medco-realm.json` that you will find in `~/medco-deployment/resources/configuration`.
- Select to import everything, and to Skip if resources already exist


#### Configure the MedCo OpenID Connect client
In the Settings tab, fill Valid Redirect URIs with the value `http://localhost/glowing-bear`.
In the same tab, fill Web Origins with + and save.


#### Add a user and attribute permissions
- Go to the configuration panel Users, click on Add user.
- Fill the Username field, toggle to ON the Email Verified button and click Save.
- In the next window, click on Credentials, enter twice the user’s password, toggle to OFF the Temporary button if desired and click Reset Password.
- Go to the configuration panel Users, search for the user you want to give authorization to and click on Edit.
- Go to the Role Mappings tab, and select medco in the Client Roles.
- Add all the roles.


### Load MedCo Demo Data
Execute the following commands to load data in the 3 nodes and restart the deployment.

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


## A Note about the Data
The used data are massive (in the order of tens of Terabytes), so we cannot host it online. In order to obtain them to reproduce the experiments, please contact juan.troncoso-pastoriza@epfl.ch.

