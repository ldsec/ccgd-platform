#!/usr/bin/env bash
set -Eeuo pipefail

OUTPUT_FILE=$1
NB_REPETITIONS=$2
QUERY=$3

USER=keycloak
PASSWORD=keycloak
QUERY_TYPE=patient_list

if [[ ! -f ${OUTPUT_FILE} ]]; then
  docker-compose run icclusters --user ${USER} --password ${PASSWORD} --disableTLSCheck --bypassPicsure q --resultFile /results/${OUTPUT_FILE} ${QUERY_TYPE} ${QUERY}
else
  ((NB_REPETITIONS++))
fi

for (( i=1; i<${NB_REPETITIONS}; i++ )); do
    docker-compose run icclusters --user ${USER} --password ${PASSWORD} --disableTLSCheck --bypassPicsure q --resultFile /results/${OUTPUT_FILE}.tmp ${QUERY_TYPE} ${QUERY}
    tail -n +2 ${OUTPUT_FILE}.tmp >> ${OUTPUT_FILE}
    rm ${OUTPUT_FILE}.tmp
done
