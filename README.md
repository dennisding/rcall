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

## 0. install the rcall

Run the following Cargo command in your project directory:

cargo add rcall

Or add the following line to your Cargo.toml:

rcall = "0.1.0"

## 1. define the protocols

```rust
#[rcall::protocol]
pub trait EchoClient {
    fn echo_back(&mut self, msg: String);
}

#[rcall::protocol]
pub trait EchoServer {
    fn echo(&mut self, msg: String);
}
```

## 2. implement and use the services for Server

```rust
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

```

## 3. implement and use the Connection for client

```rust
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

    loop {
        client.poll();
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
```
