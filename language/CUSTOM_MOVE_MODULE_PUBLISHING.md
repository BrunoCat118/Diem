
# create a move module inside Modules Folder

* diem/language/diem-framework/modules/Sample.move

```java
    module DiemFramework::Sample{
    use DiemFramework::DiemAccount;

     public fun is_zero_balance(balance:u64):bool{
        if (balance==0)
            true
        else
            false
    }

    public(script) fun peer_to_peer_with_metadata_sample<Currency>(
        payer: signer,
        payee: address,
        amount: u64,
        metadata: vector<u8>,
        metadata_signature: vector<u8>
    ) {
        let payer_withdrawal_cap = DiemAccount::extract_withdraw_capability(&payer);
        DiemAccount::pay_from<Currency>(
            &payer_withdrawal_cap, payee, amount, metadata, metadata_signature
        );
        DiemAccount::restore_withdraw_capability(payer_withdrawal_cap);
    }
}
```

* DiemFramework implies the account address in which Sample module is stored.

# Steps to compile and publish the module

```bash
git clone https://github.com/diem/diem.git
cd diem

#run devsetup script
./scripts/dev_setup.sh

#set source for Cargo
source "$HOME/.cargo/env"

#Run Validator on local
cargo run -p diem-node -- --test

#Open a new terminal and run the below command:
cargo run -p cli -- -c $CHAIN_ID -m $ROOT_KEY -u http://127.0.0.1:8080 --waypoint $WAYPOINT

#The above info is information of validator
#In the diem cli use the account and move commands to create new account and mint some coins so that the account is created on-chain.

#Create a local account--no on-chain effect. Returns reference ID to use in other operations
account c

#mint | mintb | m | mb <receiver_account_ref_id>|<receiver_account_address> <number_of_coins> <currency_code> [use_base_units (default=false)]
#        Send currency of the given type from the faucet address to the given recipient address. Creates an account at the recipient address if one does not already exist.

account mintb 0 10 XUS

#Compile the created module using the below command:
#compile | c <sender_account_address>|<sender_account_ref_id> <file_path> <dependency_source_files...>
 #       Compile Move program
dev compile diem-framework/modules/Sample.move diem/language/move-stdlib/modules diem/language/diem-framework/modules

#Note: Please use relative path for file_path and dependency_source_files.

#Publish the module using the below command where compiled module path is produced while compiling:
#publish | p <sender_account_address>|<sender_account_ref_id> <compiled_module_path>
#        Publish Move module on-chain
dev publish 0000000000000000000000000a550c18 /var/folders/8y/mz8f3kds6hl3z0n3nz6pgj280000gp/T/304ada475efc7a72b490c5de49b9330a/modules/0_Sample.mv

#Note:The account trying to publish should have module publishing access, root-user already has permission to do so.

```

# Steps to add functional tests

• Go to path /diem/language/move-lang/functional-tests/tests/diem.

• Create a move test file under test folder.

• All the test cases have written in move file.
e.g:

```rust
//! account: bob, 1000000
script {
use DiemFramework::XUS::XUS;
use DiemFramework::Sample;
use DiemFramework::DiemAccount;
use Std::Signer;
const BALANCES_ARE_NOT_EQUAL: u64 = 77;
fun main(sender: signer) {
    let recipient_addr = @{{bob}};
    let sender_addr = Signer::address_of(&sender);
    let sender_original_balance = DiemAccount::balance<XUS>(sender_addr);
    let recipient_original_balance = DiemAccount::balance<XUS>(recipient_addr);
    Sample::peer_to_peer_with_metadata_sample<XUS>(sender,recipient_addr,5,x"", x"");
    let sender_new_balance = DiemAccount::balance<XUS>(sender_addr);
    let recipient_new_balance = DiemAccount::balance<XUS>(recipient_addr);
    assert(sender_new_balance == sender_original_balance - 5, BALANCES_ARE_NOT_EQUAL);
    assert(recipient_new_balance == recipient_original_balance + 5, BALANCES_ARE_NOT_EQUAL);
}
}
```

• Use the command  ''cargo test --package move-lang-functional-tests --test functional_testsuite <filename> ''.

# Steps to add unit tests

• Go to path /diem/language/diem-framework/tests.

• Create a move test file under unit_test folder.

•   All the test cases have written in move language.
e.g:

```rust
#[test_only]
module Std::SampleTest{
    use DiemFramework::Sample;
    const IS_NOT_ZERO_BALANCE: u64 = 77;
    #[test]
    fun is_zero_balance() {
        let balance= 0;
        let expected_output=true;
        assert(Sample::is_zero_balance(balance)==expected_output,IS_NOT_ZERO_BALANCE);
    }
    }
```

• Use the command "cargo test --package diem-framework --test move_unit_test -- move_unit_tests --exact --nocapture <filename>".

# Code Changes that must be done for script,script-function to work

```rust
// Add encode and decode functions in the diem/sdk/transaction-builder/src/stdlib.rs
//sample module
pub fn encode_peer_to_peer_with_metadata_sample_script_function(
    currency: TypeTag,
    payee: AccountAddress,
    amount: u64,
    metadata: Vec<u8>,
    metadata_signature: Vec<u8>,
) -> TransactionPayload {
    TransactionPayload::ScriptFunction(ScriptFunction::new(
        ModuleId::new(
            AccountAddress::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
            ident_str!("Sample").to_owned(),
        ),
        ident_str!("peer_to_peer_with_metadata_sample").to_owned(),
        vec![currency],
        vec![
            bcs::to_bytes(&payee).unwrap(),
            bcs::to_bytes(&amount).unwrap(),
            bcs::to_bytes(&metadata).unwrap(),
            bcs::to_bytes(&metadata_signature).unwrap(),
        ],
    ))
}
```

```rust
fn decode_peer_to_peer_with_metadata_sample_script_function(
    payload: &TransactionPayload,
) -> Option<ScriptFunctionCall> {
    if let TransactionPayload::ScriptFunction(script) = payload {
        Some(ScriptFunctionCall::PeerToPeerWithMetadata {
            currency: script.ty_args().get(0)?.clone(),
            payee: bcs::from_bytes(script.args().get(0)?).ok()?,
            amount: bcs::from_bytes(script.args().get(1)?).ok()?,
            metadata: bcs::from_bytes(script.args().get(2)?).ok()?,
            metadata_signature: bcs::from_bytes(script.args().get(3)?).ok()?,
        })
    } else {
        None
    }
}
```

* Add the function in diem/sdk/src/transaction_builder.rs to call the encode functions when an API tries to use the transaction with script/scriptfunction payload.

```rust
pub fn peer_to_peer_with_metadata_sample(
        &self,
        currency: Currency,
        payee: AccountAddress,
        amount: u64,
        metadata: Vec<u8>,
        metadata_signature: Vec<u8>,
    ) -> TransactionBuilder {
        if self.is_script_function_enabled() {
            self.payload(stdlib::encode_peer_to_peer_with_metadata_sample_script_function(
                currency.type_tag(),
                payee,
                amount,
                metadata,
                metadata_signature,
            ))
        } else {
            self.script(stdlib::encode_peer_to_peer_with_metadata_script(
                currency.type_tag(),
                payee,
                amount,
                metadata,
                metadata_signature,
            ))
        }
    }
```

# Running the above script function using cli

* Make the below changes in diem/testsuite/cli/src/client_proxy.rs

```rust
//Replace the transfer_coins_int function with this function.
pub fn transfer_coins_int(
        &mut self,
        sender_address: &AccountAddress,
        receiver_address: &AccountAddress,
        num_coins: u64,
        coin_currency: String,
        gas_unit_price: Option<u64>,
        gas_currency_code: Option<String>,
        max_gas_amount: Option<u64>,
        is_blocking: bool,
    ) -> Result<IndexAndSequence> {
        let currency_code = from_currency_code_string(&coin_currency)
            .map_err(|_| format_err!("Invalid currency code {} specified", coin_currency))?;
        let gas_currency_code = gas_currency_code.or(Some(coin_currency));

        let (sender_account_ref_id, sender) = self.get_account_data_and_id(sender_address)?;
        let program = transaction_builder::encode_peer_to_peer_with_metadata_sample_script_function(
            type_tag_for_currency_code(currency_code),
            *receiver_address,
            num_coins,
            vec![],
            vec![],
        );
        let txn = self.create_txn_to_submit(
            program,
            sender,
            max_gas_amount,    /* max_gas_amount */
            gas_unit_price,    /* gas_unit_price */
            gas_currency_code, /* gas_currency_code */
        )?;
        self.submit_and_wait(&txn, is_blocking)?;

        Ok(IndexAndSequence {
            account_index: AccountEntry::Index(sender_account_ref_id),
            sequence_number: txn.sequence_number(),
        })
    }
```

* After making the changes to client_proxy please run the cli again,publish the compiled module, add accounts , add currency to accounts as mentioned in the above steps, then try the below command.
Note:We have to run cli again because we are making the changes in client_proxy.rs

```bash
#transfer | transferb | t | tb  <sender_account_address>|<sender_account_ref_id> <receiver_account_address>|<receiver_account_ref_id> <number_of_coins> <currency_code> [gas_unit_price_in_micro_diems (default=0)] [max_gas_amount_in_micro_diems (default 400_000)] Suffix 'b' is for blocking.
 # Transfer coins from one account to another.
#eg: Run the below command to transfer 10 XUS coins from(index number zero) one account to another(index number 1) account.

transferb 0 1 10 XUS
```
