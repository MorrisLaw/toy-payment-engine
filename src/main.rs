use rust_decimal::Decimal;
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

#[derive(Deserialize)]
struct Transaction {
    id: u32,
    client_id: u16,
    kind: String,
    amount: Decimal,
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    for tx in rdr.deserialize() {
        let tx: Transaction = tx?;

        match tx.kind.as_str(){
            "deposit" => println!("ClientID: {}, TransactionID: {}, Kind: {}, Amount: {}", tx.client_id.clone(), tx.id.clone(), tx.kind.clone(), tx.amount.clone()),
            "withdrawal" => println!("ClientID: {}, TransactionID: {}, Kind: {}, Amount: {}", tx.client_id.clone(), tx.id.clone(), tx.kind.clone(), tx.amount.clone()),
            "dispute" => println!("ClientID: {}, TransactionID: {}, Kind: {}, Amount: {}", tx.client_id.clone(), tx.id.clone(), tx.kind.clone(), tx.amount.clone()),
            "resolve" => println!("ClientID: {}, TransactionID: {}, Kind: {}, Amount: {}", tx.client_id.clone(), tx.id.clone(), tx.kind.clone(), tx.amount.clone()),
            "chargeback" => println!("ClientID: {}, TransactionID: {}, Kind: {}, Amount: {}", tx.client_id.clone(), tx.id.clone(), tx.kind.clone(), tx.amount.clone()),
            _ => println!("default"),
        }
    }

    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}