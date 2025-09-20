use rcall;

#[rcall::protocol]
#[allow(dead_code)]
pub trait ImplInClient {
    #[rcall::rpc(1)]
    fn hello_from_server(&mut self, msg: String);
    #[rcall::rpc(2)]
    fn login_result(&mut self, ok: i32);
}

#[rcall::protocol]
#[allow(dead_code)]
pub trait ImplInServer {
    #[rcall::rpc(1)]
    fn hello_from_client(&mut self, msg: String);
    #[rcall::rpc(2)]
    fn login(&mut self, name: String, password: String);
}