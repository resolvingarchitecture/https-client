use ra_common::models::{Network, Packet};

use log::{trace,info};

pub struct HTTPSClient {

}

impl HTTPSClient {
    pub fn new() -> HTTPSClient {
        HTTPSClient {

        }
    }
    pub fn init(&mut self) {
        info!("{}","Initializing HTTPS Client...")

    }
    pub fn handle(&mut self, packet: &mut Packet) {
        info!("Handling incoming packet id={}",packet.id);

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
