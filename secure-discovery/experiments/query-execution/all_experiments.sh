#!/usr/bin/env bash
set -Eeuo pipefail

bash experiments_run.sh exp_20r_1q_1511p.csv 20 194
bash experiments_run.sh exp_10r_10q_1511p.csv 10 194^10
bash experiments_run.sh exp_5r_50q_1511p.csv 5 194^50

bash experiments_run.sh exp_50r_1q_7p.csv 50 357

bash experiments_run.sh exp_50r_1q_52p.csv 50 97
bash experiments_run.sh exp_20r_50q_52p.csv 20 97^50

bash experiments_run.sh exp_50r_1q_150p.csv 50 106

bash experiments_run.sh exp_50r_1q_451p.csv 50 71
bash experiments_run.sh exp_10r_10q_451p.csv 10 71^10
bash experiments_run.sh exp_10r_50q_451p.csv 10 71^50

bash experiments_run.sh exp_5r_50q_940p.csv 5 46^50

bash experiments_run.sh exp_20r_1q_1366p.csv 20 183
bash experiments_run.sh exp_10r_5q_1366p.csv 10 183^5
bash experiments_run.sh exp_10r_25q_1366p.csv 10 183^25
bash experiments_run.sh exp_5r_100q_1366p.csv 5 183^100
bash experiments_run.sh exp_10r_100q_1366p.csv 10 183^100

bash experiments_run.sh exp_20r_1q_1511p.csv 20 194
bash experiments_run.sh exp_10r_10q_1511p.csv 10 194^10
bash experiments_run.sh exp_10r_50q_1511p.csv 10 194^50


bash experiments_run.sh exp_50r_1q_940p.csv 50 46
bash experiments_run.sh exp_10r_10q_940p.csv 10 46^10
bash experiments_run.sh exp_10r_50q_940p.csv 10 46^50

bash experiments_run.sh exp_50r_1q_585p.csv 50 158
bash experiments_run.sh exp_20r_5q_585p.csv 20 158^5
bash experiments_run.sh exp_10r_25q_585p.csv 10 158^25
bash experiments_run.sh exp_5r_100q_585p.csv 5 158^100
bash experiments_run.sh exp_10r_100q_585p.csv 10 158^100

