use rcall::{self};

mod protocols;
use protocols::{ImplInServer};

struct ServicesImpl {
}

impl ServicesImpl {
    pub fn new() -> Self {
        ServicesImpl {

        }
    }
}

impl rcall::ServerServices for ServicesImpl {
    type ConnectionType = ConnectionImpl;
    fn new_connection(&mut self, connection: &mut rcall::Connection) -> Self::ConnectionType {
        let remote = ConnectionRemote::new(connection.new_sender());
        ConnectionImpl::new(remote)
    }

    fn on_connected(&mut self, connection: &mut rcall::Connection, _dispatcher: &mut Self::ConnectionType) {
        println!("on client connected!{}, {}", connection.id, connection.addr);
    }

    fn on_disconnected(&mut self, connection: &mut rcall::Connection, _dispatcher: &mut Self::ConnectionType) {
        println!("on client disconnected!: {}, {}", connection.id, connection.addr);
    }
}


type ConnectionRemote = rcall::server_to_remote_type!(protocols::ImplInClient);

#[derive(rcall::Dispatcher)]
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
    }

    fn login(&mut self, name: String, password: String) {
        println!("login:name[{}], password[{}]", name, password);
        self.remote.login_result(1024);
        
//        self.remote.close();
    }
}

fn main() {
    println!("hello server!");
    let mut server = rcall::Server::new(ServicesImpl::new());

    server.serve_forever_at(999);
}