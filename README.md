# Stellar DID Registry

[DID Specification](https://w3c.github.io/did-core/)

## Overview

DID method registry for Stellar Network, built using the [Soroban Rust SDK](https://github.com/stellar/rs-soroban-sdk)

## This DID method registry contract is intended for use with the DID-Method-Resolver

### DID Format

``` did:stellr:<stellar_account_id> ```

### Contract Deployments

| Network | Address |
| ------- | ------- |
| Mainnet | Not Deployed |
| Testnet | Not Deployed |
| Futurenet | Not Deployed |

## Development

### Building

```bash

cargo build --target wasm32-unknown-unknown --release 

```

To build with logs

```bash
cargo build --target wasm32-unknown-unknown --release-with-logs
```

NOTE: if you build with logs then all future calls should use the wasm path `target/wasm32-unknown-unknown/release-with-logs/stellar_did.wasm`

### Tests

```bash
cargo test
```

### Deploying and Invoking the contract

### Option 1: run on local Soroban sandbox

```bash
soroban contract invoke \                                                                                                                                                                                                                        Py base 01:55:41 PM
--wasm target/wasm32-unknown-unknown/release/stellar_did.wasm \
--id 1 \
-- \
-- FUNCTION_NAME \
--PARAM_NAME_1 PARAM_VALUE_1 \
--PARAM_NAME_2 PARAM_VALUE_2 
```

### Option 2: deploy to local docker network

Run local network using Docker:

```bash
docker run --rm -it \
  -p 8000:8000 \
  --name stellar \
  stellar/quickstart:soroban-dev@sha256:57e8ab498bfa14c65595fbb01cb94b1cdee9637ef2e6634e59d54f6958c05bdb \
  --standalone \
  --enable-soroban-rpc
```

Generate Account:

go to [Stellar Laboratory](https://laboratory.stellar.org/#account-creator?network=test), for the rest of the commands replace G with your pubkey and S with your secret key

```bash
curl "http://localhost:8000/friendbot?addr=G..."
```

Deploy contract:

```bash
soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/[project-name].wasm \
    --source S... \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Standalone Network ; February 2017'
```
