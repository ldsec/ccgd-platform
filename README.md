# Citizen-Centered Genomic Discovery Platform
This repository contains the code and scripts needed to reproduce the experimental results reported in the article:
D. Grishin, J.L. Raisaro, J.R. Troncoso-Pastoriza, K. Obbad, G. Church, J.P. Hubaux, "Citizen-Centered, Auditable, and Privacy-Preserving Population Genomics" (the full reference will be added when the paper is online).
The code is divided in two folders:
- Secure Discovery Tools
- Consent Management and Access Control Tools

## Secure Discovery
​This is the code that corresponds to the privacy-conscious patient discovery tool, which uses and extends the core functionalities of the MedCo framework (https://medco.epfl.ch). More information about the MedCo framework and codebase, and detailed steps for its deployment can be found at the MedCo documentation: https://medco.epfl.ch/documentation/

### Deployment Profiles
This subfolder contains deployment profiles, the used keys, configuration and docker-compose files needed to run the experiments in a network configured with different numbers of nodes and database sizes.
Note that these deployments contain configuration specific to a certain set of EPFL machines, which need to be modified in order to be reproduced in other machines.

### Experiments
This subfolder contains the code needed to replicate an existing i2b2 database and extend it to reach the scales tested in the article (up to 28 billion observations), the SQL scripts to insert these data in a running database (deployed with the aforementioned deployment profiles), the scripts to run the experiments, and the logs for the obtained results.
​
### Crypto Operations
This subfolder contains the code needed to reproduce the experiments for data loading timings, including the local encryption time for an increasing number of variables, and the equality-preserving re-encryption time before an individual record is inserted in the database.

### A Note about the Data
The used data are massive (in the order of tens of Terabytes), so we cannot host it online. In order to obtain them to reproduce the experiments, please contact juan.troncoso-pastoriza@epfl.ch.

## Consent Management and Access Control Tools
TBC
