# Deterministic re-encryption experiments 

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

All the configuration for the experiments are in **this repository under** `runfiles\`

A simple example:
```bash 
./simul -platform localhost runfiles/re-encryption-6servers.toml
```

