
mod packer;
use packer::PackTo;

fn main() {
    println!("Hello, world!");

    let mut packet = packer::Packet::new();

    33_i32.pack_to(&mut packet);
    3.14_f32.pack_to(&mut packet);
    "hello world!".pack_to(&mut packet);

    println!("msg: {:?}", packet.buffer);
}
