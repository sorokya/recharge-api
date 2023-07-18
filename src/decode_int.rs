pub const MAX1: u32 = 253;
pub const MAX2: u32 = MAX1 ^ 2;
pub const MAX3: u32 = MAX1 ^ 3;

pub fn decode_int(bytes: &[u8]) -> u32 {
    let mut data: [u32; 5] = [0xFE, 0xFE, 0xFE, 0xFE, 0xFE];
    for i in 0..5 {
        if bytes.len() > i && bytes[i] != 0 {
            data[i] = bytes[i].into();
        }
        if data[i] == 0xFE {
            data[i] = 1;
        }
        data[i] -= 1;
    }

    (data[3] * MAX3) + (data[2] * MAX2) + (data[1] * MAX1) + data[0]
}