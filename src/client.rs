
pub use rcall;

mod protocols;
mod packer;
mod network;

use protocols::Client;

#[derive(rcall::Protocol)]
struct ClientImpl {

}

impl ClientImpl {
    pub fn new() -> Self {
        ClientImpl {

        }
    }
}

impl protocols::Client for ClientImpl {
    async fn hello_from_server(&mut self, msg: String) {
        println!("hello from server!!!!{}", msg);
    }
}

fn main() {
    println!("hello client!!!");
    // let client_impl = ClientImpl::new()
    // let mut client = rcall::Client::new(client_impl);
    // client.connect(ip, port);
    let mut _runtime = tokio::runtime::Runtime::new();

    let client_impl = ClientImpl::new();

    let mut client = rcall::Client::new();
    client.connect(String::from("127.0.0.1"), 999);

    loop {
        let _ = tokio::time::sleep(tokio::time::Duration::from_nanos(1));
    }
}