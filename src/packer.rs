
pub struct Packet {
    pub buffer: Vec<u8>,
    index: usize,
}

pub trait PackTo {
    fn pack_to(&self, packet: &mut Packet);
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