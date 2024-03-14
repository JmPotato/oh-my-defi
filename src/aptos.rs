use aptos_sdk::{rest_client::Client as InnerClient, types::account_address::AccountAddress};
use url::Url;

// TODO: support the devnet and testnet as well.
static APTOS_MAINNET_URL: &str = "https://fullnode.mainnet.aptoslabs.com";
static APTOS_PRECISION: f64 = 1_0000_0000.;
pub static APTOS_COIN_TYPE: &str = "0x1::aptos_coin::AptosCoin";

pub struct Client {
    inner: InnerClient,
}

impl Client {
    pub fn new() -> Self {
        Self {
            inner: InnerClient::new(Url::parse(APTOS_MAINNET_URL).unwrap()),
        }
    }

    /// Get the account balance by the given address and coin type.
    pub async fn get_account_balance(
        &self,
        address: &str,
        coin_type: &str,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let response = self
            .inner
            .get_account_balance_bcs(
                AccountAddress::from_str_strict(address)?,
                if coin_type.is_empty() {
                    APTOS_COIN_TYPE
                } else {
                    coin_type
                },
            )
            .await?;
        Ok(response.into_inner() as f64 / APTOS_PRECISION)
    }
}
