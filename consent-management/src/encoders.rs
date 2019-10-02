//! Objects that can be encoded in a format that can be stored on the blockchain.

use exonum::crypto::{Hash, PublicKey};

encoding_struct! {
    /// Cryptocurrency wallet.
    struct Wallet {
        /// Public key of the wallet owner.
        pub_key:            &PublicKey,
        /// Amount of tokens in the wallet.
        balance:            u64,
    }
}

impl Wallet {
    /// Returns a copy of the wallet with an updated balance.
    pub fn set_balance(self, balance: u64) -> Self {
        Self::new(self.pub_key(), balance)
    }
}

encoding_struct!{
    /// Object representing data stored on our platform.
    struct Data {
        /// Location of the encrypted data.
        location:            &str,
        /// Type of data (e.g. survey data, genomic data).
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
        generator_key:       &PublicKey,
        /// Hex string representing the public key of the data owner,
        /// encrypted with the symmetric key.
        /// Used to verify ownership of the data.
        encrypted_owner_key: &str,
        /// Any extra information about the data.
        metadata:            &str,
    }
}
