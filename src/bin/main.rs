extern crate log;
extern crate simple_logger;

use log::{trace,info};
use ra_common::models::Packet;

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting HTTPS Client Daemon...");
    let mut client = https_client::HTTPSClient::new();
    client.init();
    trace!("HTTPS Client Daemon Stopped.");
}
