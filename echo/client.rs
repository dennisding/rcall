
use rcall;

mod protocols;
use protocols::EchoClient;

type ClientRemote = rcall::client_to_remote_type!(protocols::EchoServer);

struct EchoClientServices {
}

impl rcall::ClientServices for EchoClientServices {
    type DispatcherType = EchoClientImpl;

    fn new_dispatcher(&mut self, sender: rcall::ClientSender) -> Self::DispatcherType {
        EchoClientImpl {
            remote: ClientRemote::new(sender)
        }
    }

    fn on_connected(&mut self, dispatcher: &mut Self::DispatcherType) {
        dispatcher.remote.echo("some 中文 message".to_string());
    }
}

#[derive(rcall::Dispatcher)]
struct EchoClientImpl {
    remote: ClientRemote
}

impl EchoClient for EchoClientImpl {
    fn echo_back(&mut self, msg: String) {
        println!("sound from server: {}", msg);
    }
}

fn main() {
    let mut client = rcall::Client::new(EchoClientServices {});
    client.connect("127.0.0.1".to_string(), 999);

    client.block_poll();

    // loop {
    //     client.poll();
    //     std::thread::sleep(std::time::Duration::from_millis(1));
    // }
}