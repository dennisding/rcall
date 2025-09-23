use rcall;

#[rcall::protocol]
#[allow(dead_code)]
pub trait ImplInClient {
    #[rcall::rpc(10)]
    fn hello_from_server(&mut self, msg: String);
    #[rcall::rpc(11)]
    fn login_result(&mut self, ok: i32);
}

#[rcall::protocol]
#[allow(dead_code)]
pub trait ImplInServer {
    #[rcall::rpc(10)]
    fn hello_from_client(&mut self, msg: String);
    #[rcall::rpc(11)]
    fn login(&mut self, name: String, password: String);
}