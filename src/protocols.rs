
use rcall;
use rcall_macro;
use crate::packer;

use packer::{PackTo, UnpackFrom};

pub trait Client {
    fn hello_from_server(&self, msg: &str);

    // generate by macro
    // rpc(1)
}

#[rcall::protocol]
pub trait Server {
    #[rcall::rpc(1)]
    fn hello_from_client(&self, msg: &str);
    #[rcall::rpc(2)]
    fn login(&self, name: &str, password: &str);

    // auto generate by macro!!!
}

impl<T: Server> crate::network::RpcDispatcher for T {
    fn dispatch_rpc(&mut self, rpc_id: i16, mut packet: packer::Packet) {
        match rpc_id {
            1 => {
                if let Some((msg)) = crate::unpack!(packet, String) {
                    self.hello_from_client(msg.as_str());
                }
            }
            2 => {
                if let Some((name, password)) = crate::unpack!(packet, String, String) {
                    self.login(name.as_str(), password.as_str())
                }
            }
            _ => {

            }
        }
    }
}