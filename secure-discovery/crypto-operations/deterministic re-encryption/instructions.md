# Deterministic re-encryption experiments 

This experiment provides the execution time for the encryption of a set of X integers (nbrEncryptions) in your local machine. This serves as a benchmark for the encryption of the query elements done by the MedCo/I2B2 client.

## Requirements 

* Git
* Golang (>1.11)

## Instructions

```bash 
git clone https://github.com/ldsec/unlynx.git
```

Build simulation executable: 

```bash 
cd unlynx/simul
go build ./...
```
Execute simulation locally based on a specific profile (`.toml`)

```bash 
./simul -platform localhost [-debug 1] {path to .toml}
```

All the configuration for the experiments are in the **ccgd-platform** under `secure-discovery/crypto-operations/deterministic re-encryption\runfiles\`

A simple example:
```bash 
./simul -platform localhost runfiles/re-encryption-6servers.toml
```

