#[derive(Debug)]
pub struct Block {
    pub number: Option<u64>,
    pub hash: Option<String>,
    pub parent_hash: Option<String>,
    pub nonce: Option<String>,
    pub sha3_uncles: Option<String>,
    pub logs_bloom: Option<String>,
    pub transactions_root: Option<String>,
    pub state_root: Option<String>,
    pub receipts_root: Option<String>,
    pub miner: Option<String>,
    pub difficulty: Option<u64>,
    pub total_difficulty: Option<u64>,
    pub size: Option<u64>,
    pub extra_data: Option<String>,
    pub gas_limit: Option<u64>,
    pub gas_used: Option<u64>,
    pub timestamp: Option<u64>,
    pub withdrawals_root: Option<String>,
    pub transactions: Vec<u64>,
    pub transaction_count: Option<u64>,
    pub base_fee_per_gas: u64,
    pub withdrawals: Vec<u64>,
}