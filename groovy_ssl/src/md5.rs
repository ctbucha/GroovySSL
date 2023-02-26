use std::cmp;
use std::fmt;

const A0: u32 = 0x67452301;
const B0: u32 = 0xefcdab89;
const C0: u32 = 0x98badcfe;
const D0: u32 = 0x10325476;

#[cfg_attr(rustfmt, rustfmt::skip)]
const S: [u32; 64] = [
     7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
     5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
     4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
     6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,
];

#[cfg_attr(rustfmt, rustfmt::skip)]
const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
    0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
    0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
    0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
    0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
    0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
    0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
    0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
    0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

#[derive(Clone, Copy)]
pub struct MD5Hash([u8; 16]);

impl IntoIterator for MD5Hash {
    type Item = u8;
    type IntoIter = std::array::IntoIter<Self::Item, 16>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Display for MD5Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.into_iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

fn md5_round(message: [u32; 16], a0: u32, b0: u32, c0: u32, d0: u32) -> (u32, u32, u32, u32) {
    let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);
    for i in 0..64 {
        let (f, g) = match i >> 4 {
            0 => ((b & c) | ((!b) & d), i),
            1 => ((d & b) | ((!d) & c), ((i * 5) + 1) % 16),
            2 => (b ^ c ^ d, ((i * 3) + 5) % 16),
            _ => (c ^ (b | (!d)), (i * 7) % 16),
        };
        let f2 = f
            .wrapping_add(a)
            .wrapping_add(K[i])
            .wrapping_add(message[g]);
        a = d;
        d = c;
        c = b;
        b = b.wrapping_add(f2.rotate_left(S[i]));
    }
    (a, b, c, d)
}

struct MessagePreprocessor<'a> {
    message: &'a [u8],
    chunk_index: usize,
}

impl MessagePreprocessor<'_> {
    fn new(message: &[u8]) -> MessagePreprocessor {
        MessagePreprocessor {
            message: message,
            chunk_index: 0,
        }
    }
}

impl Iterator for MessagePreprocessor<'_> {
    type Item = [u32; 16];

    fn next(&mut self) -> Option<Self::Item> {
        let message_length = self.message.len();
        // 1 byte end + 8 byte length
        let chunks_count = ((message_length + 1 + 8) / 64) + 1;

        if self.chunk_index >= chunks_count {
            return None;
        }

        let mut message_chunk: [u8; 64] = [0; 64];
        let message_start_index = self.chunk_index * 64;
        match if message_start_index <= message_length {
            Some(cmp::min(message_length - message_start_index, 64))
        } else {
            None
        } {
            Some(chunk_content_length) => {
                for i in 0..chunk_content_length {
                    message_chunk[i] = self.message[message_start_index + i];
                }
                if chunk_content_length < 64 {
                    message_chunk[chunk_content_length] = 0x80;
                }
                if chunk_content_length + 1 + 8 < 64 {
                    let bit_length = message_length << 3;
                    for i in 0..7 {
                        message_chunk[56 + i] = ((bit_length >> i * 8) & 0xFF).try_into().unwrap();
                    }
                }
            }
            None => {
                let bit_length = message_length << 3;
                for i in 0..7 {
                    message_chunk[56 + i] = ((bit_length >> i * 8) & 0xFF).try_into().unwrap();
                }
            }
        }
        let mut message_chunk_u32: [u32; 16] = [0; 16];
        for i in 0..16 {
            let byte0: u32 = message_chunk[i * 4].try_into().unwrap();
            let byte1: u32 = message_chunk[i * 4 + 1].try_into().unwrap();
            let byte2: u32 = message_chunk[i * 4 + 2].try_into().unwrap();
            let byte3: u32 = message_chunk[i * 4 + 3].try_into().unwrap();
            message_chunk_u32[i] = byte0 + (byte1 << 8) + (byte2 << 16) + (byte3 << 24)
        }
        self.chunk_index += 1;
        Some(message_chunk_u32)
    }
}

pub fn hash(message: &[u8]) -> MD5Hash {
    let (mut a, mut b, mut c, mut d) = (A0, B0, C0, D0);

    for message_chunk in MessagePreprocessor::new(message) {
        let (a2, b2, c2, d2) = md5_round(message_chunk, a, b, c, d);
        (a, b, c, d) = (
            a.wrapping_add(a2),
            b.wrapping_add(b2),
            c.wrapping_add(c2),
            d.wrapping_add(d2),
        );
    }

    MD5Hash([
        (a & 0xFF).try_into().unwrap(),
        ((a >> 8) & 0xFF).try_into().unwrap(),
        ((a >> 16) & 0xFF).try_into().unwrap(),
        ((a >> 24) & 0xFF).try_into().unwrap(),
        (b & 0xFF).try_into().unwrap(),
        ((b >> 8) & 0xFF).try_into().unwrap(),
        ((b >> 16) & 0xFF).try_into().unwrap(),
        ((b >> 24) & 0xFF).try_into().unwrap(),
        (c & 0xFF).try_into().unwrap(),
        ((c >> 8) & 0xFF).try_into().unwrap(),
        ((c >> 16) & 0xFF).try_into().unwrap(),
        ((c >> 24) & 0xFF).try_into().unwrap(),
        (d & 0xFF).try_into().unwrap(),
        ((d >> 8) & 0xFF).try_into().unwrap(),
        ((d >> 16) & 0xFF).try_into().unwrap(),
        ((d >> 24) & 0xFF).try_into().unwrap(),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!(
            format!("{}", hash(b"")),
            String::from("d41d8cd98f00b204e9800998ecf8427e")
        );
    }

    #[test]
    fn string1() {
        assert_eq!(
            format!("{}", hash(b"a")),
            String::from("0cc175b9c0f1b6a831c399e269772661")
        );
    }

    #[test]
    fn string2() {
        assert_eq!(
            format!("{}", hash(b"abc")),
            String::from("900150983cd24fb0d6963f7d28e17f72")
        );
    }

    #[test]
    fn string3() {
        assert_eq!(
            format!("{}", hash(b"message digest")),
            String::from("f96b697d7cb7938d525a2f31aaf161d0")
        );
    }

    #[test]
    fn string4() {
        assert_eq!(
            format!("{}", hash(b"abcdefghijklmnopqrstuvwxyz")),
            String::from("c3fcd3d76192e4007dfb496cca67e13b")
        );
    }

    #[test]
    fn string5() {
        assert_eq!(
            format!(
                "{}",
                hash(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789")
            ),
            String::from("d174ab98d277d9f5a5611c2c9f419d9f")
        );
    }

    #[test]
    fn string6() {
        assert_eq!(
            format!("{}", hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890")),
            String::from("57edf4a22be3c955ac49da2e2107b67a")
        );
    }
}
