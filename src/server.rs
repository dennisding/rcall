//use crate::protocols::Server;


// pub mod packer;
mod protocols;
use protocols::{Server};
//mod network;
use rcall::{self};

#[derive(rcall::Protocol)]
struct ConnectionImpl {
    name: String,
}

impl ConnectionImpl {
    pub fn new() -> Self {
        ConnectionImpl {
            name: String::from("name!!!")
        }
    }
}

// impl network::RpcDispatcher for ConnectionImpl {
//     async fn dispatch_rpc(&mut self, rpc_id: i32, packet: crate::packer::Packet) {
//         self._dispatch_rpc(rpc_id, packet);
//     }
// }

impl protocols::Server for ConnectionImpl {
    async fn hello_from_client(&mut self, msg: String) {
        println!("hello_from_client: {}", msg);
    }

    async fn login(&mut self, name: String, password: String) {
        println!("login: {}, {}", name, password);
    }
}

struct ServerImpl {
}

impl ServerImpl {
    pub fn new() -> Self {
        ServerImpl {

        }
    }
}

impl rcall::network::Server for ServerImpl {
    type ConnectType = ConnectionImpl;
    fn new_connection(&self) -> Self::ConnectType {
        ConnectionImpl::new()
    }

    fn on_connected(&self, connection: &Self::ConnectType) {
        println!("on_connected!");
    }

    fn on_disconnected(&self, connection: &Self::ConnectType) {
        println!("on_disconnected!");
    }
}

async fn do_serve() {
    let mut server = rcall::services!(protocols::Client, protocols::Server);
    server.serve_forever(999, ServerImpl::new()).await;
}

fn main() {
    println!("hello server");
//    let mut server = Server::new();

//    server.serve_at(999);
    // let mut server = crate::services!(protocols::Client, protocols::Server);
    // server.serve_forever(999, ServerImpl::new());
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.spawn(do_serve());

    loop {
        std::thread::sleep(std::time::Duration::from_nanos(1));
    }
}
