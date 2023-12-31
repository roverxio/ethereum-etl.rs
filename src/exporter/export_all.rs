
use serde::{Deserialize};
use tokio_postgres::Client;
use crate::json_rpc_requests::json_rpc_requests::{json_rpc_requests};
use crate::exporter::export_blocks_and_transactions::export_blocks_and_transactions;


#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse {
    jsonrpc: String,
    pub(crate) result: serde_json::Value,
    id: usize,
}

pub async fn export_all(start_block: u64, end_block: u64, provider: &str, client: &Client) -> Result<(), reqwest::Error>  {
    export_blocks_and_transactions(start_block,end_block, provider, client).await?;

    //TODO: export_reciepts_and_logs()
    //TODO: extract_token_transfers()
    //TODO: export_traces();
    //TODO: export_contracts();
    //TODO: extract_tokens();

    Ok(())
}






