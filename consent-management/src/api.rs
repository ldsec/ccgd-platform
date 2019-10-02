//! Nebula's REST API.
//!
//! Sets up endpoints for reading data, and one endpoint to process transactions.

use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    blockchain::Transaction,
    crypto::{Hash, PublicKey},
    node::TransactionSend,
};

use encoders::{Data, Wallet};
use transactions::NebulaTransactions;
use NebulaSchema;

/// Contains API methods and their endpoints.
#[derive(Debug, Clone, Copy)]
pub struct NebulaApi;

/// Query for a specific wallet.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct WalletQuery {
    /// Public key of the wallet owner.
    pub pub_key: PublicKey,
}

/// Query for data.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct DataQuery {
    /// Public key matching the data owner.
    pub pub_key: PublicKey,
}

/// Response given to transaction requests.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    /// Hash of the processed transaction.
    pub tx_hash: Hash,
}

impl NebulaApi {
    /// Gets a list of all wallets.
    fn get_wallets(state: &ServiceApiState, _query: ()) -> api::Result<Vec<Wallet>> {
        let snapshot = state.snapshot();
        let schema = NebulaSchema::new(snapshot);
        let idx = schema.wallets();
        let wallets = idx.values().collect();
        Ok(wallets)
    }

    /// Gets a specific wallet.
    fn get_wallet(state: &ServiceApiState, query: WalletQuery) -> api::Result<Wallet> {
        let snapshot = state.snapshot();
        let schema = NebulaSchema::new(snapshot);
        let pub_key = &query.pub_key;
        match schema.wallet(pub_key) {
            None => Err(api::Error::NotFound("Wallet doesn't exist".to_string())),
            Some(wallet) => Ok(wallet),
        }
    }

    /// Gets a list of Data for a public key.
    fn get_data(state: &ServiceApiState, query: DataQuery) -> api::Result<Vec<Data>> {
        let snapshot = state.snapshot();
        let schema = NebulaSchema::new(snapshot);
        let data = schema.data(&query.pub_key);
        let result = data.iter().collect();
        Ok(result)
    }

    /// Processes a transaction and forwards it to the network.
    fn post_transaction(
        state: &ServiceApiState,
        query: NebulaTransactions,
    ) -> api::Result<TransactionResponse> {
        let transaction: Box<dyn Transaction> = query.into();
        let tx_hash = transaction.hash();
        state.sender().send(transaction)?;
        Ok(TransactionResponse { tx_hash })
    }

    /// Wires endpoints to API methods.
    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/data", Self::get_data)
            .endpoint("v1/wallets", Self::get_wallets)
            .endpoint("v1/wallet", Self::get_wallet)
            .endpoint_mut("v1/transaction", Self::post_transaction);
    }
}
