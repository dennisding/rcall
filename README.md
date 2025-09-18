# rcall

a rpc framework written by rust.

rcall是一个用rust编写的单线程rpc框架.
所有的读操作都由单独的异步函数阻塞式读取.
事件的处理与写操作都在主线程进行.
这么设计的原因是为了后续能用于游戏服务器中.
游戏中的对象需要频繁更新和与环境互动,
并不适合读写和事件处理在独立的异步函数中的模式.

rcall is a single-threaded RPC framework written in Rust.
All reading operations are handled in a blocking manner by separate asynchronous functions.
Event handling and writing operations are both performed on the main thread.
The reason for this design is to allow its future use in game servers.
Game objects need to be updated frequently and interact with the environment,
which makes them unsuitable for a pattern where reading, writing, and event handling are done in separate asynchronous functions.

## how to use?

## 1. define the protocols

```rust
#[rcall::protocol]
struct ServerToClient {
    #[rcall::rpc(100)]
    fn hello_from_server(&mut self, msg: String);
    fn login_result(&mut self, is_ok: Bool);
}

#[rcall::protocol]
struct ClientToServer {
    fn hello_from_client(&mut self, msg: String);
    fn login(&mut self, name: String, password: String);
}
```

## 2. implement and use the services for Server

```rust
#[derive(rcall::Protocol)]
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
#[derive(rcall::Protocol)]
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
