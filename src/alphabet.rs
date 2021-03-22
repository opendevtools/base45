pub struct Base45 {}

const TABLE: [char; 45] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', ' ', '$',
    '%', '*', '+', '-', '.', '/', ':',
];

impl Base45 {
    pub fn decode(v: char) -> Option<usize> {
        Some(TABLE.iter().position(|&r| r == v).unwrap())
    }

    pub fn encode(n: usize) -> Option<char> {
        Some(TABLE[n])
    }
}
