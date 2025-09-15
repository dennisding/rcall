
//pub use rcall;

use tokio;

//mod client_impl;
mod protocols;
//mod packer;
//mod network;
use rcall::{self, network};

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

impl network::Client for ClientImpl {
    fn on_connected(&mut self) {
        println!("on client connected");
    }

    fn on_disconnected(&mut self) {
        println!("on client disconnected");
    }
}

impl protocols::Client for ClientImpl {
    async fn hello_from_server(&mut self, msg: String) {
        println!("hello from server!!!!{}", msg);
    }
}

async fn do_connection() {
    let client_impl = ClientImpl::new();
    let mut client = rcall::Client::new(client_impl);
    client.connect(String::from("127.0.0.1"), 999);
}

fn main() {
    println!("hello client!!!");
    // let client_impl = ClientImpl::new()
    // let mut client = rcall::Client::new(client_impl);
    // client.connect(ip, port);
    let runtime = tokio::runtime::Runtime::new().unwrap();

//    runtime
    runtime.spawn(do_connection());

    // let client_impl = ClientImpl::new();
    // let mut client = rcall::Client::new(client_impl);
    // client.connect(String::from("127.0.0.1"), 999);

    loop {
        std::thread::sleep(std::time::Duration::from_nanos(1));
//        let _ = tokio::time::sleep(tokio::time::Duration::from_nanos(1));
    }
}