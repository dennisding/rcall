# rcall

a rpc framework written by rust.

## how to use?

## 1. define the protocols

```rust
#[rcall::protocol]
struct ServerToClient {
    #[rcall::rpc(100)]
    fn hello_from_server(&mut self, msg: String);
    fn login_result(&mut self, is_ok: Bool);
}

struct ClientToServer {
    fn hello_from_client(&mut self, msg: String);
    fn login(&mut self, name: String, password: String);
}
```

## 2. implement and use the services for Server

```rust
struct Services {
}

impl rcall::network::Services for Services {
    type ConnectType = ConnectionImpl;
    fn new_connection(&mut self) -> Self::ConnectType {
        ConnectionImpl {
        }
    }
    fn on_connected(&mut self, connection: Self::ConnectType) {
        println!("on_connected!!!!");
    }
}

struct ConnectionImpl {
}

impl protocols::ClientToServer {
    fn hello_from_client(&mut self, msg: String) {
        println!("hello_from_client: msg:{}", msg);
        self.remote.hello_from_server("msg from server!!!");
    }

    fn login(&mut self, name: String, password: String) {
        println!("login: name[{}], password[{}]", name, password);
        self.remote.login_result();
    }
}

fn main() {
    let services = rcall::services!(protocols::Client, protocols::Server);
    services.serve_forever_at(999, Services::new());
}

```

## 3. implement and use the Connection for client

```rust
struct ClientImpl {

}

impl rcall::network::Client for ClientImpl {
    fn on_connected(&mut self) {
        println!("on_connected");
    }
    ...
}

impl protocols::ServerToClient for ClientImpl {
    fn hello_from_server(&mut self, msg: String) {
        println!("hello_from_server: {}", msg);
        self.remote.hello_from_client("msg from client!");
        self.remote.login(String::from("dennis"), String::from("ding"));
    }

    fn login_result(&mut self, result: bool) {
        println!("login result! {}", result);
    }
}

fn main() {
    let client = rcall::client!(protocols::ServerToClient, protocols::ClientToServer);
    client.connect_to("127.0.0.1", 999, ClientImpl::new());

    loop {
        client.process();
    }
}
```
