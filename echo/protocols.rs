

#[rcall::protocol]
#[allow(dead_code)]
pub trait EchoClient {
    fn echo_back(&mut self, msg: String);
}

#[rcall::protocol]
#[allow(dead_code)]
pub trait EchoServer {
    fn echo(&mut self, msg: String);
}