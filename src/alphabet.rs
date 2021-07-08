pub const TABLE: [u8; 45] = *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:";
pub fn decode(v: u8) -> Option<usize> {
    TABLE.iter().position(|&r| r == v)
}

pub fn encode(n: usize) -> Option<u8> {
    TABLE.get(n).map(|&c| c)
}

pub const SIZE: usize = 45;
pub const SIZE_SIZE: usize = SIZE * SIZE;
