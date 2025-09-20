
// use packer
// let packet = packer::pack!(ivalue, fvalue, "string value!");
// let result = packer::unpack!(packet, i32, f32, String);
// match result {
//      None => {}
//      Some((iv, fv, s)) => {
//          println!("unpack value");
//      }
// }

pub struct Packet {
    pub buffer: Vec<u8>,
    index: usize,
}

pub trait PackTo {
    fn pack_to(&self, packet: &mut Packet);
}

pub trait UnpackFrom {
    type OutType;
    fn unpack_from(packet: &mut Packet) -> Option<Self::OutType>;
}

impl Packet {
    pub fn new(len: usize) -> Self {
        let mut buffer = Vec::<u8>::with_capacity(len);
        unsafe {
            buffer.set_len(len);
        }

        Packet {
            buffer,
            index: 0
        }
    }
}

//#[macro_export]
macro_rules! BuildPacker {
    ($type: ty) => {
        impl PackTo for $type {
            fn pack_to(&self, packet: &mut Packet) {
                packet.buffer.extend_from_slice(&self.to_ne_bytes());
            }
        }
    };
}

// #[macro_export]
macro_rules! BuildUnpacker {
    ($type: ty) => {
        impl UnpackFrom for $type {
            type OutType = $type;
            fn unpack_from(packet: &mut Packet) -> Option<Self::OutType> {
                let size = std::mem::size_of::<$type>();
                if packet.index + size > packet.buffer.len() {
                    return None;
                }

                unsafe {
                    let ptr = packet.buffer.as_ptr().add(packet.index) as *const $type;
                    packet.index = packet.index + size;
                    return Some(std::ptr::read_unaligned(ptr));
                }
            }
        }
    }
}

#[macro_export]
macro_rules! pack {
    ($($expr: expr), *) => {{
        use rcall::packer::PackTo;
        let mut packet = rcall::packer::Packet::new(0);
        $(
            $expr.pack_to(&mut packet);
        )*
        packet
    }};
}

#[macro_export]
macro_rules! gen_values {
    ($packet: ident, $none_value: ident, ($($values: tt),* ), $head: ty, $($tails: ty),* ) => {{
        use rcall::{UnpackFrom};
        let result = <$head>::unpack_from(&mut $packet);
        if let Some(value) = result {
            rcall::gen_values!($packet, $none_value, ($($values,)* value), $($tails),* )
        } else {
            $none_value
        }
    }};

    ($packet: ident, $none_value: ident, ($($values: tt), *), $head: ty ) => {{
        use rcall::{UnpackFrom};
        let result = <$head>::unpack_from(&mut $packet);
        if let Some(value) = result {
            Some(($($values,)* value))
        } else {
            $none_value
        }
    }};
}

#[macro_export]
macro_rules! unpack {
    ($packet: ident, $($types: ty),* ) => {{
        let none_value = Option::<($($types),*)>::None;
        rcall::gen_values!($packet, none_value, (), $($types),* )
    }}
}

// build packer
BuildPacker!(i8);
BuildPacker!(i16);
BuildPacker!(i32);
BuildPacker!(i64);
BuildPacker!(u8);
BuildPacker!(u16);
BuildPacker!(u32);
BuildPacker!(u64);
BuildPacker!(usize);
BuildPacker!(f32);
BuildPacker!(f64);

// build unpacker
BuildUnpacker!(i8);
BuildUnpacker!(i16);
BuildUnpacker!(i32);
BuildUnpacker!(i64);
BuildUnpacker!(u8);
BuildUnpacker!(u16);
BuildUnpacker!(u32);
BuildUnpacker!(u64);
BuildUnpacker!(usize);
BuildUnpacker!(f32);
BuildUnpacker!(f64);

impl PackTo for str {
    fn pack_to(&self, packet: &mut Packet) {
        let size = self.len() as i16;
        size.pack_to(packet);
        packet.buffer.extend_from_slice(self.as_bytes());
    }
}

impl PackTo for String {
    fn pack_to(&self, packet: &mut Packet) {
        self.as_str().pack_to(packet);
    }
}

impl UnpackFrom for str {
    type OutType = String;
    fn unpack_from(packet: &mut Packet) -> Option<Self::OutType> {
        return String::unpack_from(packet);
    }
}

impl UnpackFrom for String {
    type OutType = String;
    fn unpack_from(packet: &mut Packet) -> Option<Self::OutType> {
        if let Some(size) = i16::unpack_from(packet) {
            if packet.index + size as usize > packet.buffer.len() {
                return None;
            }
            let size = size as usize;
            let slice = &packet.buffer[packet.index .. packet.index + size];
            unsafe {
                packet.index = packet.index + size;
                return Some(String::from_utf8_unchecked(slice.to_vec()));
            }
        }
        
        return None;
    }
}
