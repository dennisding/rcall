
use rcall;

mod protocols;
use protocols::{EchoServer};

type EchoRemote = rcall::server_to_remote_type!(protocols::EchoClient);

struct EchoServerServices { 
}

impl rcall::ServerServices for EchoServerServices {
    type ConnectionType = EchoConnection;
    fn new_connection(&mut self, connection: &mut rcall::Connection) -> Self::ConnectionType {
        EchoConnection {
            remote: EchoRemote::new(connection.new_sender()),
        }
    }
}

#[derive(rcall::Dispatcher)]
struct EchoConnection {
    remote: EchoRemote
}

impl protocols::EchoServer for EchoConnection {
    fn echo(&mut self, msg: String) {
        println!("sound from client!:{}", msg);
        self.remote.echo_back(msg);

        self.remote.close();
    }
}

fn main() {
    let mut server = rcall::Server::new(EchoServerServices {});
    server.serve_forever_at(999);
}