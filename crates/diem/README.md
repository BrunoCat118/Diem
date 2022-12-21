# Diem Command Line Interface (CLI) Tool

The `diem` tool is a command line interface (CLI) for debugging, development, and node operation.

## Invoke the Diem CLI client
To invoke the Diem CLI client, run the following command in the diem/crates/diem folder.

```bash
cargo run
```

## Account commands
Account related operations.

|Command | Description|
|----- |----- |
| `account create`| Create a new local account and returns a public/private keypair and authorization key.|
|`account mint <AUTHORIZATION KEY> <AMOUNT> <CURRENCY>` | Mint coins to an account. |

### Account Create
The account create command is used to create a new local account. The account can be created on chain by transferring coins to the created account.

```bash
cargo run account create
```

### Account Mint
The account mint command transfers newly minted coins in the specified account and creates the account if not already created on chain.

Currently, this command only works in testnet and uses the faucet to mint coins. Functionality will be added in the future to add the ability to create accounts using the treasury compliance account and mint coins with the designated dealer account.

```bash
cargo run account mint <AUTHORIZATION KEY> 100 XUS
```
