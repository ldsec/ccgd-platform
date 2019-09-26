#!/usr/bin/env bash

PGPASSWORD=i2b2 psql -v ON_ERROR_STOP=1 -h localhost -U i2b2 -d i2b2medco -f ./sensitive_tagged.sql 2>&1 > sensitive_tagged.log
PGPASSWORD=i2b2 psql -v ON_ERROR_STOP=1 -h localhost -U i2b2 -d i2b2medco -f ./concept_dimension.sql 2>&1 > concept_dimension.log
PGPASSWORD=i2b2 psql -v ON_ERROR_STOP=1 -h localhost -U i2b2 -d i2b2medco -f ./observation_fact.sql 2>&1 > observation_fact.log