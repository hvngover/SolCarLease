use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey};

pub fn connect_to_network() -> RpcClient {
    let url = "https://api.devnet.solana.com";
    RpcClient::new(url.parse().unwrap())
}

pub async fn get_account_balance(client: &RpcClient, pubkey: &Pubkey) -> Result<u64, Box<dyn std::error::Error>> {
    let balance = client.get_balance(pubkey).await?;
    Ok(balance)
}