
use tokio_postgres::{NoTls, Error, Client};
use crate::models::block::Block;

pub async fn connect_to_db() -> Client {
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=postgres dbname=ethereum_etl", NoTls).await.expect("failed to initialize client");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
        .execute(
            "CREATE TABLE IF NOT EXISTS blocks (
                number VARCHAR,
                hash VARCHAR,
                parent_hash VARCHAR,
                nonce VARCHAR,
                sha3_uncles VARCHAR,
                logs_bloom VARCHAR,
                transactions_root VARCHAR,
                state_root VARCHAR,
                receipts_root VARCHAR,
                miner VARCHAR,
                difficulty VARCHAR,
                total_difficulty VARCHAR,
                size VARCHAR,
                extra_data VARCHAR,
                gas_limit VARCHAR,
                gas_used VARCHAR,
                timestamp VARCHAR,
                withdrawals_root VARCHAR,
                transactions TEXT[],
                transaction_count VARCHAR,
                base_fee_per_gas VARCHAR,
                withdrawals VARCHAR
            )",
            &[],
        )
        .await.expect("failed to create table");
    println!("Created table blocks...");
    client

}

pub async fn insert_block_data(client: &Client, block: Block) -> Result<(), Error> {

    client
        .execute(
            "INSERT INTO blocks (
                number,
                hash,
                parent_hash,
                nonce,
                sha3_uncles,
                logs_bloom,
                transactions_root,
                state_root,
                receipts_root,
                miner,
                difficulty,
                total_difficulty,
                size,
                extra_data,
                gas_limit,
                gas_used,
                timestamp,
                withdrawals_root,
                transactions,
                transaction_count,
                base_fee_per_gas,
                withdrawals
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)",
            &[
                &block.number,
                &block.hash,
                &block.parent_hash,
                &block.nonce,
                &block.sha3_uncles,
                &block.logs_bloom,
                &block.transactions_root,
                &block.state_root,
                &block.receipts_root,
                &block.miner,
                &block.difficulty,
                &block.total_difficulty,
                &block.size,
                &block.extra_data,
                &block.gas_limit,
                &block.gas_used,
                &block.timestamp,
                &block.withdrawals_root,
                &block.transactions,
                &block.transaction_count,
                &block.base_fee_per_gas,
                &block.withdrawals,
            ],
        )
        .await.expect("could not insert data");
    println!("Inserted block data...");

    Ok(())
}