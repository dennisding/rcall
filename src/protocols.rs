use rcall;
//use crate::packer;

// use rcall::packer::{PackTo, UnpackFrom};

// #[rcall::protocol]
// pub trait Client {
//     async fn hello_from_server(&mut self, msg: String);
// }

// #[rcall::protocol]
// pub trait Server {
//     #[rcall::rpc(1)]
//     async fn hello_from_client(&mut self, msg: String);
//     #[rcall::rpc(2)]
//     async fn login(&mut self, name: String, password: String);
// }


#[rcall::protocol]
pub trait ImplInClient {
    #[rcall::rpc(1)]
    fn hello_from_server(&mut self, msg: String);
    #[rcall::rpc(2)]
    fn login_result(&mut self, ok: i32);
}

#[rcall::protocol]
pub trait ImplInServer {
    #[rcall::rpc(1)]
    fn hello_from_client(&mut self, msg: String);
    #[rcall::rpc(2)]
    fn login(&mut self, name: String, password: String);
}