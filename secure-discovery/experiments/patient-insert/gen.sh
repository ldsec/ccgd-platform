#!/usr/bin/env bash

for i in {111..1000110}; do

    echo "('2', '\\medco\\tagged\\NEB-${i}\\', '', 'N', 'LA', '0', 'concept_cd', 'concept_dimension', 'concept_path', 'T', 'LIKE', '\\medco\\tagged\\NEB-${i}\\', '', '', 'NOW()', 'NOW()', 'NOW()', 'TAG_ID', '@', 'TAG_ID:${i}')," >> sensitive_tagged.sql
    echo "('\medco\tagged\NEB-${i}\', 'TAG_ID:${i}', 'NOW()', '1')," >> concept_dimension.sql
    echo "('1', '1', 'TAG_ID:${i}', 'e2etest', 'NOW()', '@', '1', 'NOW()', '1')," >> observation_fact.sql

done
