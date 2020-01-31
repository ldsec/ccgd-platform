# Blockchain for consent management and data access controls

This repo contains the code needed to spin-up an example blockchain and validator nodes, as well as the set of example API calls for this blockchain.  

## Install and run

Run blockchain nodes locally.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://crates.io/)

### Launch script

```console
./launch.sh
```

This script launches four nodes.

- Public endpoints: `127.0.0.1:8000` ... `127:0.0.1:8003`
- Private endpoints: `127.0.0.1:8004` ... `127.0.0.1:8007`

Nodes log at error level by default.
Change the log level by passing in the `RUST_LOG` environment variable.

```console
RUST_LOG=info ./launch.sh
```

### Manually

An example of what the launch script does under the hood, but with only two nodes.

#### Generate configs

Generate a template config `common.toml` for two validator nodes.

```console
cargo run -- generate-template common.toml --validators-count 2 --backend-key 8970eee89a78fae63e32ced013d07e248ae10cd1c5726f706b1559677d771d99
```

Generate public and secret keys for each node, storing them in `pub_N.toml` and `sec_N.toml`.

```console
cargo run -- generate-config common.toml  pub_1.toml sec_1.toml --peer-address 127.0.0.1:7000

cargo run -- generate-config common.toml  pub_2.toml sec_2.toml --peer-address 127.0.0.1:7001
```

Finalize configs using a node's secret config with values from the public configs for all nodes, and save the result to `node_N_cfg.toml`.

```console
cargo run -- finalize --public-api-address 0.0.0.0:8000 --private-api-address 0.0.0.0:8002 sec_1.toml node_1_cfg.toml --public-configs pub_1.toml pub_2.toml

cargo run -- finalize --public-api-address 0.0.0.0:8001 --private-api-address 0.0.0.0:8003 sec_2.toml node_2_cfg.toml --public-configs pub_1.toml pub_2.toml
```

#### Run nodes

```console
RUST_LOG=info cargo run -- run --node-config node_1_cfg.toml --db-path db1

RUST_LOG=info cargo run -- run --node-config node_2_cfg.toml --db-path db2
```

- Public endpoints: `127.0.0.1:8000` and `127:0.0.1:8001`
- Private endpoints: `127.0.0.1:8002` and `127.0.0.1:8003`

## Read API

### Get data

Get a list of data objects associated with a public key.

#### HTTP request

`GET /api/services/neb-chain/v1/data`

#### Query parameters

| Parameter | Type       | Description                      |
| :-------- | :--------- | :------------------------------- |
| pub_key   | Public Key | The public key of the data owner |

#### Example

##### Request

```console
curl http://127.0.0.1:8000/api/services/neb-chain/v1/data?pub_key=7d82e3b31019f64d7746a429273df1940703d92176f5f8114891ef4028df50ec
```

##### Response

```json
[
  {
    "consent_type": "yes",
    "data_type": "Yoko Ono interpretations",
    "encrypted_owner_key": "010010101010001001",
    "generator_key": "e11fa39dc9ad094dc2f88ae4e22d1d578d88e9d9f2e0cea109a6b7d1d444a1cc",
    "data_hash": "cb738e51bfe313d953dc60059e7333d38345b89a1f058e9b10bbef24f993b0ec",
    "level": "47",
    "location": "s3/test",
    "metadata": "is so meta even this acronym",
    "owner_key": "7d82e3b31019f64d7746a429273df1940703d92176f5f8114891ef4028df50ec",
    "symmetric_key": "TBD"
  }
]
```

### Get wallets

Get a list of all wallets in the blockchain.

#### HTTP request

`GET /api/services/neb-chain/v1/wallets`

#### Example

##### Request

```console
curl http://127.0.0.1:8000/api/services/neb-chain/v1/wallets
```

##### Response

```json
[
  {
    "balance": "42",
    "pub_key": "6f54a0485c59d9899db3e4c191572e09dc6222db6a185d23ecdb2fdae162d8a4"
  },
  {
    "balance": "47",
    "pub_key": "7d82e3b31019f64d7746a429273df1940703d92176f5f8114891ef4028df50ec"
  }
]
```

### Get wallet

Get information about a specific wallet.

#### HTTP request

`GET /api/services/neb-chain/v1/wallet`

#### Query parameters

| Parameter | Type       | Description                        |
| :-------- | :--------- | :--------------------------------- |
| pub_key   | Public Key | The public key of the wallet owner |

#### Example

##### Request

```console
curl http://127.0.0.1:8000/api/services/neb-chain/v1/wallet?pub_key=6f54a0485c59d9899db3e4c191572e09dc6222db6a185d23ecdb2fdae162d8a4
```

##### Response

```json
{
  "balance": "42",
  "pub_key": "6f54a0485c59d9899db3e4c191572e09dc6222db6a185d23ecdb2fdae162d8a4"
}
```

## Transactions

All transactions are sent using the same endpoint.

`POST /api/services/neb-chain/v1/transaction`

They also have shared request parameters.

| Parameter        | Type                   | Description                                                         |
| :--------------- | :--------------------- | :------------------------------------------------------------------ |
| body             | JSON object            | An object containing specfic transaction parameters                 |
| message_id       | u16                    | The ID of the transaction                                           |
| protocol_version | u8                     | The major version of the Exonum serialization protocol, currently 0 |
| service_id       | u16                    | The ID of the service, for neb-chain this is 128                    |
| signature        | [Ed25519 signature][1] | The binary serialization of the message, signed with a private key  |

[1]: https://ed25519.cr.yp.to

### CreateWallet

Add a new wallet to the blockchain.

#### Body parameters

| Parameter | Type       | Description                        |
| :-------- | :--------- | :--------------------------------- |
| pub_key   | Public Key | The public key of the wallet owner |

#### Example

##### Request

```json
{
  "body": {
    "pub_key": "6f54a0485c59d9899db3e4c191572e09dc6222db6a185d23ecdb2fdae162d8a4"
  },
  "message_id": 0,
  "protocol_version": 0,
  "service_id": 128,
  "signature": "658aaa089fbccd16396807af0e7bf6cad8523656fd5545e4bf732815596d8c10d4ce15e9877e9513a6c8400a87404502df30a34551c6ff979c992849b1ea0404"
}
```

##### Response

```json
{
  "tx_hash": "1befcac18fb6ccf1cff202ffb970b99f95b7860314d6b4c7abc226fb68292eff"
}
```

### Transfer

Transfer tokens from one wallet to another.

#### Body parameters

| Parameter | Type       | Description                                             |
| :-------- | :--------- | :------------------------------------------------------ |
| from_key  | Public Key | The public key of the wallet sending tokens             |
| to_key    | Public Key | The public key of the wallet recieving tokens           |
| amount    | u64        | The amount of tokens to transfer                        |
| seed      | u64        | A randomly generated number to keep transactions unique |

#### Example

##### Request

```json
{
  "body": {
    "from_key": "6f54a0485c59d9899db3e4c191572e09dc6222db6a185d23ecdb2fdae162d8a4",
    "to_key": "7d82e3b31019f64d7746a429273df1940703d92176f5f8114891ef4028df50ec",
    "amount": "2",
    "seed": "0"
  },
  "protocol_version": 0,
  "service_id": 128,
  "message_id": 1,
  "signature": "c3f0a59d4c36c6d36ffb518aa858273b90e26367b5cd75dcbd9f85ba91ff2e1ec2960a4716c7276a2d46ce22c8a4ff92d18b702d61182b7047c8a4a4e9097605"
}
```

##### Response

```json
{
  "tx_hash": "02eeefd0002a3e9480513f0715b7a67f6f7192550a861421d342ed5170c26779"
}
```

### Issue

Add tokens to a wallet.

#### Body parameters

| Parameter | Type       | Description                                             |
| :-------- | :--------- | :------------------------------------------------------ |
| pub_key   | Public Key | The public key of the wallet to add tokens to           |
| amount    | u64        | The amount of tokens to issue                           |
| seed      | u64        | A randomly generated number to keep transactions unique |

#### Example

##### Request

```json
{
  "body": {
    "pub_key": "6f54a0485c59d9899db3e4c191572e09dc6222db6a185d23ecdb2fdae162d8a4",
    "amount": "10",
    "seed": "0"
  },
  "message_id": 2,
  "protocol_version": 0,
  "service_id": 128,
  "signature": "d8bf659a1751a5f8feee51030182eba2471e2dc436cc9cfe47ac43ecab4fb2ca0d6e17fb08f8ce2da4771cfdc4cb834caa2c3adbea96528313d456af0a154307"
}
```

##### Response

```json
{
  "tx_hash": "34a1d6d1ae53376b9968ca7518c52fcf964653ebfabc60fb0549328f8bd6b762"
}
```

### StoreData

Store data in the blockchain.

#### Body parameters

| Parameter           | Type       | Description                                                           |
| :------------------ | :--------- | :-------------------------------------------------------------------- |
| location            | String     | The location of the encrypted data                                    |
| data_type           | String     | The type of the data                                                  |
| level               | String     | The encryption level of the data                                      |
| consent_type        | String     | The type of consent provided for the data                             |
| owner_key           | Public Key | The public key of the data owner                                      |
| symmetric_key       | String     | A hex string of the encrypted symmetric key                           |
| data_hash           | Hash       | A SHA-256 hash of the data                                            |
| generator_key       | Public Key | The public key of the data generator                                  |
| encrypted_owner_key | String     | A hex string of the data owner's key encrypted with the symmetric key |
| metadata            | String     | Any extra information about the data                                  |

#### Example

##### Request

```json
{
  "body": {
    "location": "s3/test",
    "data_type": "Yoko Ono interpretations",
    "level": "47",
    "consent_type": "yes",
    "owner_key": "7d82e3b31019f64d7746a429273df1940703d92176f5f8114891ef4028df50ec",
    "symmetric_key": "TBD",
    "data_hash": "cb738e51bfe313d953dc60059e7333d38345b89a1f058e9b10bbef24f993b0ec",
    "generator_key": "e11fa39dc9ad094dc2f88ae4e22d1d578d88e9d9f2e0cea109a6b7d1d444a1cc",
    "encrypted_owner_key": "010010101010001001",
    "metadata": "is so meta even this acronym"
  },
  "message_id": 3,
  "protocol_version": 0,
  "service_id": 128,
  "signature": "5281782955ccb2b5c7c6cd73868c62824045ef120a02a3e83117f7eb110fc9a5477e2e30a9c684068c1b8e0f2be943b9b7863cd233297757746e801f056cca06"
}
```

##### Response

```json
{
  "tx_hash": "1ccaff130933b6e34e1ca83d8b702a9230ef02a397060a2289e0f372c83301a0"
}
```
