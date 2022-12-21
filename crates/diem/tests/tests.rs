// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0
use clap::Parser;
use diem::Tool;

#[tokio::test]
async fn test_all_commands() {
    let diem = "diem".to_string();
    let account = "account".to_string();

    //An authorization key must be supplied for the account mint tests to run successfully
    // let auth_key = "".to_string();

    // let amount = 10;
    // let currency = "XUS".to_string();
    //diem account create
    run_command(vec![diem.clone(), account.clone(), "create".to_string()]).await;

    //diem account mint <ACCOUNT/AUTHKEY> <AMOUNT> <CURRENCY>
    // run_command(vec![
    //     diem.clone(),
    //     account.clone(),
    //     "mint".to_string(),
    //     auth_key,
    //     amount.to_string(),
    //     currency,
    // ])
    // .await;
}

async fn run_command(arguments: Vec<String>) {
    let tool = Tool::try_parse_from(arguments).unwrap();

    match tool.execute().await {
        Ok(_) => {}
        Err(err) => {
            panic!("Error occurred during test: {}", err)
        }
    }
}
