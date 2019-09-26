#!/usr/bin/env bash
set -Eeuo pipefail

bash experiments_run.sh exp_20r_1q_1511p.csv 20 194
bash experiments_run.sh exp_10r_10q_1511p.csv 10 194^10

bash experiments_run.sh exp_50r_1q_7p.csv 50 357

bash experiments_run.sh exp_50r_1q_52p.csv 50 97
bash experiments_run.sh exp_20r_50q_52p.csv 20 97^50

bash experiments_run.sh exp_50r_1q_150p.csv 50 106

bash experiments_run.sh exp_50r_1q_451p.csv 50 71

bash experiments_run.sh exp_50r_1q_585p.csv 50 158

