//! Exonum node configuration.
//!
//! Enables service configuration generation by extending command line methods.

use exonum::{
    crypto::PublicKey,
    encoding::serialize::decode_hex,
    helpers::fabric::{keys, Argument, CommandExtension, Context},
    node::NodeConfig,
};
use failure;
use toml::Value;

use SERVICE_NAME;

/// Object containing Nebula's service configuration.
#[derive(Clone, Deserialize, Serialize)]
pub struct NebulaConfig {
    /// Public key used to verify transactions originating from a web backend.
    pub backend_pub_key: PublicKey,
}

/// Object we use to extend the `generate-template` CLI command.
pub struct GenerateCommonConfig;

impl CommandExtension for GenerateCommonConfig {
    /// Sets up extra command line arguments.
    fn args(&self) -> Vec<Argument> {
        // Required: backend public key.
        vec![Argument::new_named(
            "BACKEND_KEY",
            true,
            "Backend public key",
            None,
            "backend-key",
            false,
        )]
    }

    /// Takes extra command line arguments and adds them to neb-chain's service config.
    fn execute(&self, mut context: Context) -> Result<Context, failure::Error> {
        // Read what was passed in as the BACKEND_KEY parameter, should be a hex string.
        let arg = context.arg::<String>("BACKEND_KEY").unwrap();
        // Attempt to decode the BACKEND_KEY into binary.
        let decoded =
            &decode_hex(arg).expect("Unable to decode the BACKEND_KEY hex string into binary");
        // Attempt to convert the binary into a PublicKey object.
        let backend_pub_key =
            PublicKey::from_slice(&decoded).expect("BACKEND_KEY is not a valid public key");
        // Construct a NebulaConfig object using the public key.
        let nebula_config = NebulaConfig { backend_pub_key };

        // Grab the services_config section from the context.
        let mut services_config = context.get(keys::SERVICES_CONFIG).unwrap();

        // Add Nebula's config to the neb-chain section of the services_config.
        services_config.insert(
            String::from(SERVICE_NAME),
            Value::try_from(nebula_config).expect("Could not serialize Nebula's service config"),
        );

        context.set(keys::SERVICES_CONFIG, services_config);
        Ok(context)
    }
}

/// Object we use to extend the `finalize` CLI command.
pub struct Finalize;

impl CommandExtension for Finalize {
    /// Sets up extra command line arguments, currently none.
    fn args(&self) -> Vec<Argument> {
        vec![]
    }

    /// The Finalize implementation just throws out any existing services_config values, this fixes that.
    fn execute(&self, mut context: Context) -> Result<Context, failure::Error> {
        // Grab node and common configurations.
        let mut node_config: NodeConfig = context.get(keys::NODE_CONFIG).unwrap();
        let common_config = context.get(keys::COMMON_CONFIG).unwrap();
        // Put any common service configuration into the node's service configuration.
        node_config.services_configs = common_config.services_config;
        context.set(keys::NODE_CONFIG, node_config);
        Ok(context)
    }
}
