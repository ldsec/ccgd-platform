//! Entrypoint for our application.
//! Adds services to a node, then runs it.

extern crate exonum;
extern crate exonum_configuration;
extern crate neb_chain;

use exonum::helpers::{self, fabric::NodeBuilder};
use exonum_configuration as configuration;

use neb_chain::NebulaServiceFactory;

fn main() {
    exonum::crypto::init();
    // Initialize an env_logger
    helpers::init_logger().unwrap();

    // Start a Node containing our service, and a configuration service
    let node = NodeBuilder::new()
        .with_service(Box::new(configuration::ServiceFactory))
        .with_service(Box::new(NebulaServiceFactory));
    node.run();
}
