pub const TABLE: [u8; 45] = *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:";
pub fn decode(v: u8) -> Option<u8> {
    TABLE.iter().position(|&r| r == v).map(|i| i as _)
}

pub fn encode(n: u8) -> Option<u8> {
    TABLE.get(n as usize).map(|&c| c)
}

pub const SIZE: u32 = 45;
pub const SIZE_SIZE: u32 = SIZE * SIZE;
