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
        println!("new_connection");
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
```

## 3. implement and use the Connection for client

```rust
struct ClientImpl {

}

impl rcall::network::Client {
    fn on_connected(&mut self) {
        println!("on_connected");
    }
    ...
}

impl protocols::ServerToClient {
    fn hello_from_server(&mut self, msg: String) {
        println!("hello_from_server: {}", msg);
        self.remote.hello_from_client("msg from client!");
        self.remote.login(String::from("dennis"), String::from("ding"));
    }

    fn login_result(&mut self, result: bool) {
        println!("login result! {}", result);
    }
}
```
