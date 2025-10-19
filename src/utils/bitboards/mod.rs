pub mod utils;

pub mod masks {
    pub const FILE_LEFT: u64 = 0x0101010101010101;
    pub const FILE_RIGHT: u64 = 0x8080808080808080;
    pub const FILE_TOP: u64 = 0xFF00000000000000;
    pub const FILE_BOTTOM: u64 = 0x00000000000000FF;
}