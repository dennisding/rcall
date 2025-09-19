
pub type RpcId = i32;
type ConnectId = usize;
type PacketLenType = i16;

pub const CHANNEL_SIZE: usize = 512;
pub const CONNECT_ID_BEGIN: usize = 100;

// mod message;
// pub use message::Message;

pub mod services_impl;
pub use services_impl::Server;
pub use services_impl::Connection;
pub use services_impl::ServerSender;
pub use services_impl::ConnectionInfo;

pub mod client_impl;
pub use client_impl::Client;
pub use client_impl::ClientSender;

mod bichannel;
pub use bichannel::Bichannel;

//pub mod network;

pub use rcall_macro::rpc;
pub use rcall_macro::protocol;
pub use rcall_macro::protocol_impl;
pub use rcall_macro::client_to_remote_type;
pub use rcall_macro::server_to_remote_type;
pub use rcall_macro::Dispatcher;

pub mod packer;
pub use packer::{Packet, PackTo, UnpackFrom};

pub trait Sender {
    fn send(&mut self, packet: Packet);
    fn close(&mut self);
}

// services trait need to be redesign
pub trait ServerServices {
    type ConnectionType: RpcDispatcher;
    fn new_connection(&mut self, connection: &mut Connection) -> Self::ConnectionType;
    fn on_connected(&mut self, _connection: &mut Connection, _dispatcher: &mut Self::ConnectionType) {}
    fn on_disconnected(&mut self, _connection: &mut Connection, _dispatcher: &mut Self::ConnectionType) {}
}

pub trait RpcDispatcher {
    fn dispatch_rpc(&mut self, packet: Packet);
}

pub trait ClientServices {
    type DispatcherType: RpcDispatcher;
    fn new_dispatcher(&mut self, sender: ClientSender) -> Self::DispatcherType;
    fn on_connected(&mut self, _dispatcher: &mut Self::DispatcherType) {}
    fn on_disconnected(&mut self, _dispatcher: &Self::DispatcherType) {}
}