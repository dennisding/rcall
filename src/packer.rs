
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
    pub fn new() -> Self {
        Packet {
            buffer: Vec::new(),
            index: 0
        }
    }
}

// builtin types
impl PackTo for i16 {
    fn pack_to(&self, packet: &mut Packet) {
        packet.buffer.extend_from_slice(&self.to_ne_bytes());
    }
}

impl PackTo for i32 {
    fn pack_to(&self, packet: &mut Packet) {
        packet.buffer.extend_from_slice(&self.to_ne_bytes());
    }
}

impl PackTo for f32 {
    fn pack_to(&self, packet: &mut Packet) {
        packet.buffer.extend_from_slice(&self.to_ne_bytes());
    }
}

impl PackTo for str {
    fn pack_to(&self, packet: &mut Packet) {
        let size = self.len() as i16;
        size.pack_to(packet);
        packet.buffer.extend_from_slice(self.as_bytes());
    }
}

// unpack
impl UnpackFrom for i32 {
    type OutType = i32;
    fn unpack_from(packet: &mut Packet) -> Option<Self::OutType> {
        if packet.index + 4 > packet.buffer.len() {
            return None;
        }
        unsafe {
            let ptr = packet.buffer.as_ptr().add(packet.index) as *const i32;

            packet.index = packet.index + 4;
            return Some(std::ptr::read_unaligned(ptr));
        }
    }
}

impl UnpackFrom for f32 {
    type OutType = f32;

    fn unpack_from(packet: &mut Packet) -> Option<Self::OutType> {
        if packet.index + 4 > packet.buffer.len() {
            return None;
        }

        unsafe {
            let ptr = packet.buffer.as_ptr().add(packet.index) as *const f32;
            packet.index = packet.index + 4;
            return Some(std::ptr::read_unaligned(ptr));
        }
    }
}