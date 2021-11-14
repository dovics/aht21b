const POLYNOMIAL: u8 = 0x31;
const INIT_VALUE: u8 = 0xFF;
pub fn check_crc8(message: &Vec<u8>) -> u8 {
    let mut remainder = INIT_VALUE;
    for j in 0..message.len() {
        remainder ^= message[j];
        for _ in 0..8 {
            if remainder & 0x80 != 0 {
                remainder = (remainder << 1) ^ POLYNOMIAL;
            } else {
                remainder <<= 1;
            }
        }
    }

    remainder
}