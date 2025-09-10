
use crate::packer;

use packer::{PackTo, UnpackFrom};

pub trait Client {
    fn hello_from_server(&self, msg: &str);

    // generate by macro
    // rpc(1)
    fn _call_hello_from_server(&self, mut packet: packer::Packet) {
        if let Some((msg)) = crate::unpack!(packet, String) {
            self.hello_from_server(msg.as_str());
        }
    }

    fn _pack_hello_from_server(&self, msg: &str) {

    }
}

pub trait Server {
    fn hello_from_client(&self, msg: &str);
    fn login(&self, name: &str, password: &str);
}
