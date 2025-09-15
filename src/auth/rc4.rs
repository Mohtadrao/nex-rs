
// Simple RC4 implementation (for compatibility/testing).
// Note: RC4 is cryptographically weak — this is included only to mirror behavior from nex-go for interoperability.
// This implementation provides encrypt/decrypt (they're identical) and returns Vec<u8> results.
pub struct Rc4 {
    s: [u8; 256],
    i: u8,
    j: u8,
}

impl Rc4 {
    pub fn new(key: &[u8]) -> Self {
        let mut s = [0u8; 256];
        for i in 0..256u16 {
            s[i as usize] = i as u8;
        }
        let mut j: u8 = 0;
        for i in 0..256u16 {
            let idx = i as usize;
            j = j.wrapping_add(s[idx]).wrapping_add(key[(i as usize) % key.len()]);
            s.swap(idx, j as usize);
        }
        Self { s, i: 0, j: 0 }
    }

    fn next_byte(&mut self) -> u8 {
        self.i = self.i.wrapping_add(1);
        self.j = self.j.wrapping_add(self.s[self.i as usize]);
        let si = self.s[self.i as usize];
        let sj = self.s[self.j as usize];
        self.s.swap(self.i as usize, self.j as usize);
        let t = si.wrapping_add(sj);
        self.s[t as usize]
    }

    pub fn process(&mut self, data: &mut [u8]) {
        for b in data.iter_mut() {
            let k = self.next_byte();
            *b ^= k;
        }
    }

    pub fn process_vec(mut self, mut data: Vec<u8>) -> Vec<u8> {
        self.process(&mut data);
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc4_encrypt_decrypt() {
        let key = b"secretkey";
        let plain = b"hello world".to_vec();
        let mut rc = Rc4::new(key);
        let cipher = rc.process_vec(plain.clone());
        // decrypt by creating new RC4 with same key and processing cipher
        let mut rc2 = Rc4::new(key);
        let mut dec = cipher.clone();
        rc2.process(&mut dec);
        assert_eq!(dec, plain);
    }
}
