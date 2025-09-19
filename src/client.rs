
use rcall;

mod protocols;
use protocols::ImplInClient;

type ClientRemote = rcall::client_to_remote_type!(protocols::ImplInServer);

struct ClientServicesImpl {
}

impl ClientServicesImpl {
    pub fn new() -> Self {
        ClientServicesImpl {

        }
    }
}

impl rcall::ClientServices for ClientServicesImpl {
    type DispatcherType = ClientImpl;

    fn new_dispatcher(&mut self, sender: rcall::ClientSender) -> Self::DispatcherType {
        ClientImpl::new(ClientRemote::new(sender))
    }
    
    fn on_connected(&mut self, dispatcher: &mut Self::DispatcherType) {
        println!("on client connected");
        dispatcher.remote.hello_from_client("msg from client!".to_string());
//        self.remote.hello_from_client("msg from client!".to_string());
    }

    fn on_disconnected(&mut self, _dispatcher: &Self::DispatcherType) {
        println!("on client disconnected");
    }
}

#[derive(rcall::Dispatcher)]
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

impl ImplInClient for ClientImpl {
    fn hello_from_server(&mut self, msg: String) {
        println!("hello_from_server:msg[{}]", msg);
        self.remote.login("dennis".to_string(), "ding".to_string())
    }

    fn login_result(&mut self, ok: i32) {
        println!("login_result: ok[{}]", ok);
    }
}

fn main() {
    println!("hello client!");

    let mut client = rcall::Client::new(ClientServicesImpl::new());
    client.connect("127.0.0.1".to_string(), 999);

    loop {
        client.poll();

        std::thread::sleep(std::time::Duration::from_nanos(1));
    }
}
