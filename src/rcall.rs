
pub type RpcId = i32;
type ConnectId = usize;
type PacketLenType = i16;

pub const CHANNEL_SIZE: usize = 512;
pub const CONNECT_ID_BEGIN: usize = 100;

// mod message;
// pub use message::Message;

pub mod services_impl;
pub use services_impl::Server;
pub use services_impl::Services;
pub use services_impl::Connection;
pub use services_impl::ServerSender;
pub use services_impl::RpcDispatcher;
pub use services_impl::ConnectionInfo;

pub mod client_impl;
pub use client_impl::Client;
pub use client_impl::ClientSender;
pub use client_impl::ClientServices;

mod bichannel;
pub use bichannel::Bichannel;

//pub mod network;

pub use rcall_macro::rpc;
pub use rcall_macro::protocol;
pub use rcall_macro::protocol_impl;
pub use rcall_macro::Protocol;

pub mod packer;
pub use packer::{Packet, PackTo, UnpackFrom};
