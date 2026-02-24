pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_WIDTH: usize = 64;

const RAM_SIZE:usize = 4096;
const NUM_REGS: usize = 16;
pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],    
    screen: [bool; SCREEN_WIDTH*SCREEN_HEIGHT]

}