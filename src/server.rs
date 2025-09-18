// //use crate::protocols::Server;


// // pub mod packer;
// mod protocols;
// use protocols::{Server};
// //mod network;
// use rcall::{self};

// #[derive(rcall::Protocol)]
// struct ConnectionImpl {
//     name: String,
// }

// impl ConnectionImpl {
//     pub fn new() -> Self {
//         ConnectionImpl {
//             name: String::from("name!!!")
//         }
//     }
// }

// // impl network::RpcDispatcher for ConnectionImpl {
// //     async fn dispatch_rpc(&mut self, rpc_id: i32, packet: crate::packer::Packet) {
// //         self._dispatch_rpc(rpc_id, packet);
// //     }
// // }

// impl protocols::Server for ConnectionImpl {
//     async fn hello_from_client(&mut self, msg: String) {
//         println!("hello_from_client: {}", msg);
//         // self.remote.hello_from_server(String::from("msg from server!"));
//     }

//     async fn login(&mut self, name: String, password: String) {
//         println!("login: {}, {}", name, password);
//     }
// }

// struct ServerImpl {
// }

// impl ServerImpl {
//     pub fn new() -> Self {
//         ServerImpl {

//         }
//     }
// }

// impl rcall::network::Server for ServerImpl {
//     type ConnectType = ConnectionImpl;
//     fn new_connection(&self) -> Self::ConnectType {
//         ConnectionImpl::new()
//     }

//     fn on_connected(&self, connection: &Self::ConnectType) {
//         println!("on_connected!");
//     }

//     fn on_disconnected(&self, connection: &Self::ConnectType) {
//         println!("on_disconnected!");
//     }
// }

// async fn do_serve() {
//     let mut server = rcall::services!(protocols::Client, protocols::Server);
//     server.serve_forever(999, ServerImpl::new()).await;
// }

// fn main() {
//     println!("hello server");
// //    let mut server = Server::new();

// //    server.serve_at(999);
//     // let mut server = crate::services!(protocols::Client, protocols::Server);
//     // server.serve_forever(999, ServerImpl::new());
//     let runtime = tokio::runtime::Runtime::new().unwrap();
//     runtime.spawn(do_serve());

//     loop {
//         std::thread::sleep(std::time::Duration::from_nanos(1));
//     }
// }

use rcall;

use crate::protocols::{ImplInClient, ImplInServer};
mod protocols;

struct ServicesImpl {

}

impl ServicesImpl {
    pub fn new() -> Self {
        ServicesImpl {

        }
    }
}

impl rcall::Services for ServicesImpl {
    type ConnectionType = ConnectionImpl;
    fn new_connection(&mut self, connection: &rcall::Connection) -> Self::ConnectionType {
        ConnectionImpl {
            remote: ConnectionRemote::new(connection.new_sender())
        }
    }

    fn on_connected(&mut self, info: &mut rcall::ConnectionInfo<Self::ConnectionType>) {
        let connection = &info.connection;
        println!("on client connected!{}, {}", connection.id, connection.addr);
//        info.dispatcher.remote.hello_from_server("msg from server".to_string());
    }

    fn on_disconnected(&mut self, connection: &rcall::Connection) {
        println!("on client disconnected!: {}, {}", connection.id, connection.addr);
    }
}

#[derive(rcall::Protocol)]
struct ConnectionImpl {
    remote: ConnectionRemote
}

impl ConnectionImpl {
    pub fn new(remote: ConnectionRemote) -> Self {
        ConnectionImpl {
            remote
        }
    }
}

impl protocols::ImplInServer for ConnectionImpl {
    fn hello_from_client(&mut self, msg: String) {
        println!("hello_from_client:msg = {}", msg);
        self.remote.hello_from_server(String::from("msg from server"));
//        self.remote.login_result(1024);
    }

    fn login(&mut self, name: String, password: String) {
        println!("login:name[{}], password[{}]", name, password);
        self.remote.login_result(1024);
    }
}

// implement by macro
struct ConnectionRemote {
    sender: rcall::ServerSender,
}

impl ConnectionRemote {
    pub fn new(sender: rcall::ServerSender) -> Self {
        ConnectionRemote {
            sender
        }
    }
}

impl protocols::ImplInClient for ConnectionRemote {
    fn hello_from_server(&mut self, msg: String) {
        let rpc_id: rcall::RpcId = 1;
        let packet = rcall::pack!(rpc_id, msg);
        self.sender.send(packet);
    }

    fn login_result(&mut self, ok: i32) {
        let rpc_id: rcall::RpcId = 2;
        let packet = rcall::pack!(rpc_id, ok);
        self.sender.send(packet);
    }
}

fn main() {
    println!("hello server!");
    let mut server = rcall::Server::new(ServicesImpl::new());

    server.serve_forever_at(999);
}