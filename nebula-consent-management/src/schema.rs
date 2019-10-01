//! Nebula's schema for interacting with the blockchain.
//!
//! Provides all the read and write methods on our database tables.

use exonum::{
    crypto::{Hash, PublicKey},
    storage::{Fork, ProofListIndex, ProofMapIndex, Snapshot},
};

use encoders::{Data, Wallet};
use transactions::StoreData;
use INITIAL_BALANCE;

/// Schema object containing the database view.
#[derive(Debug)]
pub struct NebulaSchema<T> {
    /// Database view. Either a read-only `Snapshot` or readable and writable `Fork`.
    view: T,
}

/// Table containing wallets.
const WALLETS: &str = "nebula.wallets";
/// Table containing data.
const DATA: &str = "nebula.data";

/// Implements the read-only methods.
impl<T> NebulaSchema<T>
where
    T: AsRef<dyn Snapshot>,
{
    /// Constructs schema from the database view.
    pub fn new(view: T) -> Self {
        NebulaSchema { view }
    }

    /// Returns a map of all wallets, keys are the wallet owner's public keys.
    pub fn wallets(&self) -> ProofMapIndex<&T, PublicKey, Wallet> {
        ProofMapIndex::new(WALLETS, &self.view)
    }

    /// Returns the wallet for a given public key.
    pub fn wallet(&self, pub_key: &PublicKey) -> Option<Wallet> {
        self.wallets().get(pub_key)
    }

    /// Returns a list of data, indexed by the public key of the data owner.
    pub fn data(&self, pub_key: &PublicKey) -> ProofListIndex<&T, Data> {
        ProofListIndex::new_in_family(DATA, pub_key, &self.view)
    }

    /// Returns the state hash of service database.
    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.wallets().merkle_root()]
    }
}

/// Implements the methods that can write to the database.
impl<'a> NebulaSchema<&'a mut Fork> {
    /// Returns a mutable map of all wallets.
    pub fn wallets_mut(&mut self) -> ProofMapIndex<&mut Fork, PublicKey, Wallet> {
        ProofMapIndex::new(WALLETS, &mut self.view)
    }

    /// Increases the balance of a wallet.
    pub fn increase_wallet_balance(&mut self, wallet: Wallet, amount: u64) {
        let wallet = {
            let balance = wallet.balance();
            wallet.set_balance(balance + amount)
        };
        self.wallets_mut().put(wallet.pub_key(), wallet.clone());
    }

    /// Decreases the balance of a wallet
    pub fn decrease_wallet_balance(&mut self, wallet: Wallet, amount: u64) {
        let wallet = {
            let balance = wallet.balance();
            wallet.set_balance(balance - amount)
        };
        self.wallets_mut().put(wallet.pub_key(), wallet.clone());
    }

    /// Creates a new wallet and adds it to the database.
    pub fn create_wallet(&mut self, pub_key: &PublicKey) {
        let wallet = Wallet::new(pub_key, INITIAL_BALANCE);
        self.wallets_mut().put(pub_key, wallet);
    }

    /// Returns a mutable list of data indexed by a public key.
    pub fn data_mut(&mut self, pub_key: &PublicKey) -> ProofListIndex<&mut Fork, Data> {
        ProofListIndex::new_in_family(DATA, pub_key, &mut self.view)
    }

    /// Adds data to the list for a public key.
    pub fn store_data(&mut self, input_data: &StoreData) {
        let data = Data::new(
            input_data.location(),
            input_data.data_type(),
            input_data.level(),
            input_data.consent_type(),
            input_data.owner_key(),
            input_data.symmetric_key(),
            input_data.data_hash(),
            input_data.generator_key(),
            input_data.encrypted_owner_key(),
            input_data.metadata(),
        );

        let pub_key = input_data.owner_key();
        self.data_mut(pub_key).push(data);
    }
}
