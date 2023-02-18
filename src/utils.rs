use std::fmt::Write;
use std::string::String;

pub fn bin2hex(input: &Vec<u8>) -> String {
    let mut res: String = String::new();
    for b in input {
        write!(res, "{:02x}", b).expect("Unable to decode binary into hex.");
    }

    res
}

pub fn deobfuscate(obfuscate_key: &Vec<u8>, data: &Vec<u8>) -> Vec<u8> {
    // Extend obfuscate key as needed
    let mut obkey: Vec<u8> = Vec::new();
    while data.len() > obkey.len() {
        for b in obfuscate_key {
            obkey.push(*b);
            if obkey.len() >= data.len() {
                break;
            }
        }
    }

    // Perform xor operation
    let mut res: Vec<u8> = Vec::new();
    for i in 0..data.len() {
        res.push(data[i] ^ obkey[i]);
    }

    res
}

pub fn read_chunk(data: &Vec<u8>, offset: &usize) -> (u64, usize) {
    let length = get_vint(&data, &offset);
    let chunk: u64 = decode_vint(&length);
    let new_offset: usize = *offset + length.len();

    (chunk, new_offset)
}

pub fn get_vint(data: &Vec<u8>, offset: &usize) -> Vec<u8> {
    // Initialize
    let mut res: Vec<u8> = Vec::new();

    // Start reading bytes
    for x in *offset as u16..data.len() as u16 {
        res.push(data[x as usize]);

        // Check if 8th bit not set
        if (data[x as usize] & 0b1000_0000) == 0 {
            return res;
        }
    }

    // Unable to read
    res
}

pub fn decode_vint(data: &Vec<u8>) -> u64 {
    let mut n: u64 = 0;
    for b in data {
        n = n << 7;

        n = n | (b & 127) as u64;
        //if (b & 0b1000_0000) == 0 {
        if b & 128 != 0 {
            n = n + 1;
        }
    }

    n
}

pub fn convert_amount(input: u64) -> u64 {
    // Check for zero
    if input == 0 {
        return input;
    }

    // Decompress
    let e = (input - 1) % 10;
    let num = (input + 1) / 10;

    // If remainder less than 9
    let mut amount: f64;
    if e < 9 {
        let d: f64 = num as f64 % 9.0;
        amount = (num as f64 / 9.0) * 10.0 + d + 1.0;
    } else {
        amount = num as f64 + 1.0;
    }

    // Get final amount
    let base: f64 = 10.0;
    amount = amount * (base.powf(e as f64));

    amount as u64
}
