# Encryption Experiments 

This experiment provides the execution time for the encryption of a set of X integers (`nbrEncryptions`) in your local machine. This serves as a benchmark for the encryption of the query elements done by the MedCo/I2B2 client. 

## Requirements 

* Git
* Golang (>1.11)

## Instructions

```bash 
git clone https://github.com/ldsec/unlynx.git
```
* Change the `nbrEncryptions` variable in the `TestEncryptIntVector()` function. (`lib/crypto_test.go`)

```bash 
cd unlynx/lib
go test -run TestEncryptIntVector 
```
