//! Nebula's transactions and transaction accessories.
//!
//! Contains transaction verification and processing logic.

use exonum::{
    blockchain::{ExecutionError, ExecutionResult, Transaction},
    crypto::{Hash, PublicKey},
    messages::Message,
    storage::Fork,
};

use backend_pub_key;
use schema::NebulaSchema;
use NEBULA_SERVICE_ID;

/// Enumerate all the errors our API will return.
#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    #[fail(display = "Wallet already exists")]
    WalletAlreadyExists = 0,

    #[fail(display = "Sender doesn't exist")]
    SenderNotFound = 1,

    #[fail(display = "Receiver doesn't exist")]
    ReceiverNotFound = 2,

    #[fail(display = "Insufficient currency amount")]
    InsufficientCurrencyAmount = 3,

    #[fail(display = "Generator doesn't exist")]
    GeneratorNotFound = 4,

    #[fail(display = "Owner doesn't exist")]
    OwnerNotFound = 5,
}

/// Converts between service-specific errors and the standard error type
/// that can be emitted by transactions.
impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}

transactions! {
    /// Nebula's transactions.
    /// These objects correspond to the `body` element of transaction requests.
    ///
    /// The order these are defined matters.
    /// When making a transaction request, the `message_id` element identifies the transaction.
    /// The `message_id` represents the position in this macro (starting at 0).
    pub NebulaTransactions {
        const SERVICE_ID = NEBULA_SERVICE_ID;

        /// Create a wallet.
        ///
        /// message_id: 0
        struct CreateWallet {
            /// The public key of the wallet owner.
            pub_key: &PublicKey,
        }

        /// Transfer tokens from one wallet to another.
        ///
        /// message_id: 1
        struct Transfer {
            /// Public key of the owner of the wallet that's sending tokens.
            from_key: &PublicKey,
            /// Public key of the owner of the wallet that's recieving tokens.
            to_key:   &PublicKey,
            /// Amount of tokens to transfer.
            amount:   u64,
            /// Randomly generated number to keep transactions unique.
            seed:     u64,
        }

        /// Issue tokens to a wallet.
        ///
        /// message_id: 2
        struct Issue {
            /// Public key of the wallet owner
            pub_key: &PublicKey,
            /// Amount of tokens to issue.
            amount:  u64,
            /// Randomly generated number to keep transactions unique.
            seed:    u64,
        }

        /// Store data in the blockchain.
        ///
        /// message_id: 3
        struct StoreData {
            /// Location of the encrypted data.
            location:            &str,
            /// Type of data.
            data_type:           &str,
            /// The data encryption level.
            level:               &str,
            /// The type of consent provided for the data.
            consent_type:        &str,
            /// Public key of the data owner.
            owner_key:           &PublicKey,
            /// Hex string representing the symmetric key encrypted by the Collective Authority.
            /// It was used to encrypt the data stored at `location`.
            // @TODO figure out how to make this a list of strings.
            symmetric_key:       &str,
            /// SHA-256 hash of the data.
            data_hash:           &Hash,
            /// Public key of the data generator.
            // Make generator_key optional once https://github.com/exonum/exonum/pull/1004 is released
            generator_key:       &PublicKey,
            /// Hex string representing the public key of the data owner,
            /// encrypted with the symmetric key.
            /// Used to verify ownership of the data.
            encrypted_owner_key: &str,
            /// Any extra information about the data.
            metadata:            &str,
        }
    }
}

impl Transaction for CreateWallet {
    fn verify(&self) -> bool {
        self.verify_signature(&backend_pub_key())
    }

    fn execute(&self, fork: &mut Fork) -> ExecutionResult {
        let mut schema = NebulaSchema::new(fork);
        let pub_key = self.pub_key();

        // Check if wallet already exists.
        if schema.wallet(pub_key).is_none() {
            schema.create_wallet(pub_key);
            Ok(())
        } else {
            Err(Error::WalletAlreadyExists)?
        }
    }
}

impl Transaction for Transfer {
    fn verify(&self) -> bool {
        // Make sure the sender is different than the reciever.
        (self.from_key() != self.to_key()) && self.verify_signature(self.from_key())
    }

    fn execute(&self, fork: &mut Fork) -> ExecutionResult {
        let mut schema = NebulaSchema::new(fork);

        // Check that the sender's wallet exists.
        let sender = schema
            .wallet(self.from_key())
            .ok_or(Error::SenderNotFound)?;
        // Check that the receiver's wallet exists.
        let receiver = schema
            .wallet(self.to_key())
            .ok_or(Error::ReceiverNotFound)?;

        let amount = self.amount();
        // Make sure the sender has enough tokens.
        if sender.balance() < amount {
            Err(Error::InsufficientCurrencyAmount)?
        }

        schema.decrease_wallet_balance(sender, amount);
        schema.increase_wallet_balance(receiver, amount);
        Ok(())
    }
}

impl Transaction for Issue {
    fn verify(&self) -> bool {
        self.verify_signature(&backend_pub_key())
    }

    fn execute(&self, fork: &mut Fork) -> ExecutionResult {
        let mut schema = NebulaSchema::new(fork);
        let pub_key = self.pub_key();

        // Check that the wallet exists.
        if let Some(wallet) = schema.wallet(pub_key) {
            let amount = self.amount();
            schema.increase_wallet_balance(wallet, amount);
            Ok(())
        } else {
            Err(Error::ReceiverNotFound)?
        }
    }
}

impl Transaction for StoreData {
    fn verify(&self) -> bool {
        self.verify_signature(self.generator_key())
    }

    fn execute(&self, fork: &mut Fork) -> ExecutionResult {
        let mut schema = NebulaSchema::new(fork);

        // Check that the data generator's wallet exists
        if schema.wallet(self.generator_key()).is_none() {
            Err(Error::GeneratorNotFound)?
        }

        // Check that the data owner's wallet exists
        if schema.wallet(self.owner_key()).is_none() {
            Err(Error::OwnerNotFound)?
        }

        schema.store_data(self);
        Ok(())
    }
}
