
use rcall;

mod protocols;
use protocols::ImplInClient;

use crate::protocols::ImplInServer;

#[derive(rcall::Protocol)]
struct ClientImpl {
    remote: ClientRemote
}

impl ClientImpl {
    pub fn new(remote: ClientRemote) -> Self {
        ClientImpl {
            remote
        }
    }
}

impl rcall::ClientServices for ClientImpl {
    fn on_connected(&mut self) {
        println!("on client connected");
        self.remote.hello_from_client("msg from client!".to_string());
    }

    fn on_disconnected(&mut self) {
        println!("on client disconnected");
    }
}

impl ImplInClient for ClientImpl {
    fn hello_from_server(&mut self, msg: String) {
        println!("hello_from_server:msg[{}]", msg);
        self.remote.login("dennis".to_string(), "ding".to_string())
    }

    fn login_result(&mut self, ok: i32) {
        println!("login_result: ok[{}]", ok);
    }
}

// implement by client
struct ClientRemote {
    sender: rcall::ClientSender,
}

impl ClientRemote {
    pub fn new(sender: rcall::ClientSender) -> Self {
        ClientRemote {
            sender
        }
    }
}

impl ImplInServer for ClientRemote {
    fn hello_from_client(&mut self, msg: String) {
        let rpc_id: rcall::RpcId = 1; 
        let packet = rcall::pack!(rpc_id, msg);
        self.sender.send(packet);
    }

    fn login(&mut self, name: String, password: String) {
        let rpc_id: rcall::RpcId = 2;
        let packet = rcall::pack!(rpc_id, name, password);
        self.sender.send(packet);
    }
}

fn main() {
    println!("hello client!");

    // generate by macro
    let mut client = rcall::Client::new();
    let dispatcher = ClientImpl::new(ClientRemote::new(client.new_sender()));
    client.set_dispatcher(dispatcher);

    client.connect("127.0.0.1".to_string(), 999);

    loop {
        client.poll();

        std::thread::sleep(std::time::Duration::from_nanos(1));
    }
}