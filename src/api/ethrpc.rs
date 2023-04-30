use serde_json::json;

pub fn estimate_gas() {
    println!("Estimate Gas RPC call");
}

pub fn block_number() -> String {
    json!({
         "jsonrpc": "2.0",
         "method": "eth_blockNumber",
         "params": [],
         "id": 83,
    })
    .to_string()
}
