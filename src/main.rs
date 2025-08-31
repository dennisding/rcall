
mod packer;
use packer::PackTo;
use packer::UnpackFrom;


fn main() {
    println!("Hello, world!");

    let mut packet = packer::Packet::new();

    33_i32.pack_to(&mut packet);
    3.14_f32.pack_to(&mut packet);
    "hello world!".pack_to(&mut packet);

    println!("msg: {:?}", packet.buffer);

    let i = i32::unpack_from(&mut packet);
    let f = f32::unpack_from(&mut packet);
    println!("unpack: {}, {}", i.unwrap(), f.unwrap());
}
