
use tokio;

mod protocols;
use rcall::{self, network};

use protocols::{Client, Server};

#[derive(rcall::Protocol)]
struct ClientImpl {
    remote: ServerRemote
}

impl ClientImpl{
    pub fn new(remote: ServerRemote) -> Self {
        ClientImpl {
            remote
        }
    }
}

impl network::Client for ClientImpl {
    fn on_connected(&mut self) {
        println!("on client connected");
//        self.remote.hello_from_client(String::from("msg from client!"));
        self.remote._hello_from_client(String::from("msg from client!"));

        self.remote._login(String::from("dennis"), String::from("ding"));
    }

    fn on_disconnected(&mut self) {
        println!("on client disconnected");
    }
}

impl protocols::Client for ClientImpl {
    async fn hello_from_server(&mut self, msg: String) {
        println!("hello from server!!!!{}", msg);
        // self.remote.login(String::from("dennis"), String::from("password"));
    }
}

struct ServerRemote {
    sender: rcall::client_impl::Sender
}

impl ServerRemote {
    pub fn new(sender: rcall::client_impl::Sender) -> ServerRemote {
        ServerRemote {
            sender
        }
    }

    fn _hello_from_client(&mut self, msg: String) {
        let rpc_id: i32 = 1;
        let packet = rcall::pack!(rpc_id, msg);
        self.sender.send(packet);
    }

    fn _login(&mut self, name: String, password: String) {
        let rpc_id: i32 = 2;
        let packet = rcall::pack!(rpc_id, name, password);
        self.sender.send(packet);
    }
}

impl protocols::Server for ServerRemote {
    async fn hello_from_client(&mut self, msg: String) {
        let packet = rcall::pack!(msg);
        self.sender.send(packet);
    }

    async fn login(&mut self, name: String, password: String) {
        let packet = rcall::pack!(name, password);
        self.sender.send(packet);
    }
}

async fn process() {
    let mut client = rcall::Client::<ClientImpl>::new();

    let sender = client.new_sender();
    let server_remote = ServerRemote::new(sender);
    let client_impl = ClientImpl::new(server_remote);

    client.set_processor(client_impl);

    client.connect(String::from("127.0.0.1"), 999);

    loop {
        client.tick().await;
        std::thread::sleep(std::time::Duration::from_nanos(1));
    }
}

fn main() {
    println!("hello client!!!");

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.spawn(process());

    loop {
        std::thread::sleep(std::time::Duration::from_nanos(1));
    }
}