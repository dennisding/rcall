
mod protocols;

struct ClientRemote {

}

impl ClientRemote {
    fn new() -> Self {
        ClientRemote {

        }
    }
}

impl protocols::Client for ClientRemote {
    fn hello_from_server(&self, msg: &str) {
        println!("hello_from_server: {}", msg);
    }
}

struct Services {
}

impl Services {
    fn remote(&self) -> Box<dyn protocols::Client> {
        return Box::new(ClientRemote::new());
    }
}

impl protocols::Server for Services {
    fn hello_from_client(&self, msg: &str) {
        println!("hello_from_client: {}", msg);
    }

    fn login(&self, name: &str, password: &str) {
        println!("login: name {}, password {}", name, password);

        self.remote().hello_from_server("msg");
    }
}

fn main() {
    println!("hello server");
}


