# Deterministic re-encryption experiments 

This experiment provides the execution time for the tagging of a set of X integers in your local machine. The configutation parameters are listed in the `runfiles/*.toml` files (one for each scenario with a different number of nodes and a fixed number of 200000 elemtents to tag). 

If you want to build up your own experiment with a different number of nodes and/or elements to be tagged you can simply change the `Host` and `NbrResponses` variables in the config (`.toml`) file.

```
Hosts, NbrResponses, NbrGroupAttributes, NbrAggrAttributes, Proofs
9, 200000, 1, 1, false
```

**Note** - `NbrGroupAttributes`, `NbrAggrAttributes` and `Proofs` should be always set to 1, 1, false respectively. 

This serves as a benchmark for the tagging of the query elements, which done when executing a query in MedCo.

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

