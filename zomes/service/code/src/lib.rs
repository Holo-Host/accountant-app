#![feature(try_from)]
#[macro_use]
extern crate hdk;
#[macro_use]
extern crate holochain_core_types_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate multihash;

use serde_json::Value;

pub use self::service_cycle::{ServiceCycle, ServiceMetrics};

pub mod service_cycle;
pub mod util;

define_zome! {
    entries: [
        service_cycle::definition()
    ]

    genesis: || {
        Ok(())
    }

    functions: {
        main (Public) {
            log_service: {
                inputs: |
                    agent_key: String,
                    request: Value,
                    response: Value,
                    metrics: service_cycle::ServiceMetrics
                |,
                outputs: |unit: ()|,
                handler: service_cycle::log_service
            }
        }
    }
}
