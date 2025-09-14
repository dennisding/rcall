use rcall;
use crate::packer;

use packer::{PackTo, UnpackFrom};

pub trait Client {
    async fn hello_from_server(&mut self, msg: String);
}

#[rcall::protocol]
pub trait Server {
    #[rcall::rpc(1)]
    async fn hello_from_client(&mut self, msg: String);
    #[rcall::rpc(2)]
    async fn login(&mut self, name: String, password: String);
}
