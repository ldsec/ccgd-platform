//! Nebula Genomics' blockchain implementation.
//!
//! It processes cryptocurrency transactions as well as data storage.
//!
//! Backed by the exonum framework.

#[macro_use]
extern crate exonum;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

pub mod api;
pub mod config;
pub mod encoders;
pub mod schema;
pub mod transactions;

use exonum::{
    api::ServiceApiBuilder,
    blockchain::{Service, ServiceContext, Transaction, TransactionSet},
    crypto::{gen_keypair, Hash, PublicKey},
    encoding::{serialize::json::reexport::Value, Error as EncodingError},
    helpers::fabric::{
        self, keys, Command, CommandExtension, CommandName, Context, ServiceFactory,
    },
    messages::RawTransaction,
    node::NodeConfig,
    storage::{Fork, Snapshot},
};
use serde_json::to_value;

use std::sync::Mutex;

use config::{Finalize, GenerateCommonConfig, NebulaConfig};
use transactions::NebulaTransactions;

pub use schema::NebulaSchema;

/// Unique service ID. Used to identify the service when making transactions.
pub const NEBULA_SERVICE_ID: u16 = 128;
/// Name of the service.
pub const SERVICE_NAME: &str = "neb-chain";
/// Initial balance of the wallet. "You get nothing!" - Willy Wonka
pub const INITIAL_BALANCE: u64 = 0;

/// Laziliy initialized static variables.
lazy_static! {
    /// Public key used to verify requests signed by a web backend.
    static ref BACKEND_KEY: Mutex<PublicKey> = Mutex::new(gen_keypair().0);
}

/// Returns the current backend public key.
pub fn backend_pub_key() -> PublicKey {
    *BACKEND_KEY.lock().unwrap()
}

/// Updates the backend public key.
fn set_backend_pub_key(backend_key: PublicKey) {
    *BACKEND_KEY.lock().unwrap() = backend_key;
}

/// Represents Nebula's service, which we register with an exonum node.
pub struct NebulaService {
    /// Configuration values for the Nebula service.
    pub config: NebulaConfig,
}

impl Service for NebulaService {
    /// Returns the name of the service.
    fn service_name(&self) -> &str {
        SERVICE_NAME
    }

    /// Returns the service identifier.
    fn service_id(&self) -> u16 {
        NEBULA_SERVICE_ID
    }

    /// Gets a list of root hashes of tables from the schema.
    /// The hashes determine the current state of the database.
    fn state_hash(&self, view: &dyn Snapshot) -> Vec<Hash> {
        let schema = NebulaSchema::new(view);
        schema.state_hash()
    }

    /// Converts incoming raw messages into transactions we can process.
    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<dyn Transaction>, EncodingError> {
        NebulaTransactions::tx_from_raw(raw).map(Into::into)
    }

    /// Adds Nebula's service configuration to the node when it starts.
    fn initialize(&self, _: &mut Fork) -> Value {
        // Set the initial backend public key.
        set_backend_pub_key(self.config.backend_pub_key);
        to_value(self.config.clone()).unwrap()
    }

    /// Runs after every block commit.
    fn after_commit(&self, context: &ServiceContext) {
        // Update the backend public key from the current config.
        let config: NebulaConfig =
            serde_json::from_value(context.actual_service_config(self).clone()).unwrap();
        set_backend_pub_key(config.backend_pub_key);
    }

    /// Wires up the Nebula REST API.
    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        api::NebulaApi::wire(builder);
    }
}

/// Factory that configures an instance of the Nebula service.
#[derive(Debug)]
pub struct NebulaServiceFactory;

impl ServiceFactory for NebulaServiceFactory {
    /// Returns the name of the service.
    fn service_name(&self) -> &str {
        SERVICE_NAME
    }

    /// Register CommandExtensions for all commands we extended.
    fn command(&mut self, command: CommandName) -> Option<Box<dyn CommandExtension>> {
        Some(match command {
            v if v == fabric::GenerateCommonConfig.name() => Box::new(GenerateCommonConfig),
            v if v == fabric::Finalize.name() => Box::new(Finalize),
            _ => return None,
        })
    }

    /// Creates an instance of the service.
    fn make_service(&mut self, context: &Context) -> Box<dyn Service> {
        // Get Nebula's configuration out of the node's .toml configuration file
        let node_config: NodeConfig = context.get(keys::NODE_CONFIG).unwrap();
        let config: NebulaConfig = node_config.services_configs[SERVICE_NAME]
            .clone()
            .try_into()
            .unwrap();

        Box::new(NebulaService { config })
    }
}
