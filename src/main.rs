use rust_decimal::Decimal;
use serde::Deserialize;
use std::io;

#[derive(Deserialize)]
struct Transaction {
    id: u32,
    client_id: u16,
    kind: String,
    amount: Decimal,
}

fn main() -> Result<(), csv::Error> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

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
