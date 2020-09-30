mod aes;
mod counter;

pub use self::aes::*;
pub use self::counter::*;

use std::collections::HashMap;

pub fn hex_to_bytes(s: &str) -> Vec<u8> {
    assert_eq!(s.len() % 2, 0);

    let mut bytes = Vec::with_capacity(s.len() / 2);
    for i in 0..s.len() / 2 {
        bytes.push(u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap());
    }
    bytes
}

pub fn bytes_to_hex(b: &[u8]) -> String {
    let mut s = String::with_capacity(b.len() * 2);
    for byte in b {
        s.push_str(&format!("{:02x}", byte));
    }
    s
}

pub fn xor_single<T>(b: &T, key: u8) -> impl Iterator<Item = u8> + '_
where
    T: AsRef<[u8]> + ?Sized,
{
    b.as_ref().iter().map(move |b| b ^ key)
}

pub fn xor_repeating(buffer: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(key.len() != 0);

    let mut outbuffer = Vec::with_capacity(buffer.len());
    for i in 0..buffer.len() {
        outbuffer.push(buffer[i] ^ key[i % key.len()]);
    }
    outbuffer
}

pub fn chisquare_frequency_score(input: &HashMap<&u8, usize>) -> f32 {
    let mut total = 0.0;

    for (&&b, &count) in input {
        let freq = count as f32 / input.len() as f32;
        let local_score = match b {
            b' ' => (freq - 0.18401).powi(2) / 0.18401,
            b'e' => (freq - 0.12702).powi(2) / 0.12702,
            b't' => (freq - 0.9056).powi(2) / 0.9056,
            b'a' => (freq - 0.8167).powi(2) / 0.8167,
            b'o' => (freq - 0.7507).powi(2) / 0.7507,
            b'i' => (freq - 0.6966).powi(2) / 0.6966,
            b'n' => (freq - 0.6749).powi(2) / 0.6749,
            b's' => (freq - 0.6327).powi(2) / 0.6327,
            b'h' => (freq - 0.6094).powi(2) / 0.6094,
            b'r' => (freq - 0.5987).powi(2) / 0.5987,
            b'd' => (freq - 0.4253).powi(2) / 0.4253,
            b'l' => (freq - 0.4025).powi(2) / 0.4025,
            b'c' => (freq - 0.2782).powi(2) / 0.2782,
            b'u' => (freq - 0.2758).powi(2) / 0.2758,
            b'm' => (freq - 0.2406).powi(2) / 0.2406,
            b'w' => (freq - 0.2360).powi(2) / 0.2360,
            b'f' => (freq - 0.2228).powi(2) / 0.2228,
            b'g' => (freq - 0.2015).powi(2) / 0.2015,
            b'y' => (freq - 0.1974).powi(2) / 0.1974,
            b'p' => (freq - 0.1929).powi(2) / 0.1929,
            b'b' => (freq - 0.1492).powi(2) / 0.1492,
            b'v' => (freq - 0.0978).powi(2) / 0.0978,
            b'k' => (freq - 0.0772).powi(2) / 0.0772,
            b'j' => (freq - 0.0153).powi(2) / 0.0153,
            b'x' => (freq - 0.0150).powi(2) / 0.0150,
            b'q' => (freq - 0.0095).powi(2) / 0.0095,
            b'z' => (freq - 0.0074).powi(2) / 0.0074,
            _ => (freq - 0.0).powi(2) / 0.0001,
        };

        total += local_score;
    }

    total
}

pub fn hamming_distance_byte(a: u8, b: u8) -> u32 {
    (a ^ b).count_ones()
}

pub fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    assert_eq!(a.len(), b.len());

    let mut distance: usize = 0;
    for i in 0..a.len() {
        distance += hamming_distance_byte(a[i], b[i]) as usize;
    }
    distance
}

pub fn crack_single_xor(buffer: &[u8]) -> u8 {
    let mut best_score = std::f32::INFINITY;
    let mut best_key = 0;
    for key in 1..=255 {
        let deciphered: Vec<_> = xor_single(&buffer, key)
            .map(|b| b.to_ascii_lowercase())
            .collect();
        let n = chisquare_frequency_score(&deciphered.as_slice().counts());
        if n < best_score {
            best_score = n;
            best_key = key;
        }
    }

    best_key
}

pub fn pkcs7_pad(buffer: &mut Vec<u8>, blocksize: usize) {
    let mut padding = blocksize - (buffer.len() % blocksize);
    if padding == 0 {
        padding = 16;
    }
    for _ in 0..padding {
        buffer.push(padding as u8);
    }
}