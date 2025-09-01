
mod packer;
use packer::PackTo;
use packer::UnpackFrom;

fn main() {
    println!("Hello, world!");

    let mut packet = packer::Packet::new();

    33_i32.pack_to(&mut packet);
    3.14_f32.pack_to(&mut packet);
    "hello world!".pack_to(&mut packet);

    let i = i32::unpack_from(&mut packet);
    let f = f32::unpack_from(&mut packet);
    let s = String::unpack_from(&mut packet);
    println!("unpack: {}, {}, {}", i.unwrap(), f.unwrap(), s.unwrap());

    let mut pack2 = pack!(22_i32, 1.7_f32, "string data!!!!");
    let result = unpack!(pack2, i32, f32, String);
    if let None = result {
        panic!("invalid result!!!");
    }
    let (ii, ff, ss) = result.unwrap();
    println!("unwrap: {}, {}, {}", ii, ff, ss);
}
