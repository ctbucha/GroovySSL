use std::cmp;

const A0: u32 = 0x67452301;
const B0: u32 = 0xefcdab89;
const C0: u32 = 0x98badcfe;
const D0: u32 = 0x10325476;

const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

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

pub fn md5(message: &[u8]) -> [u8; 16] {
    let message_length = message.len();
    // 1 byte end + 2 byte length
    let chunks_count = ((message_length + 3) / 64) + 1;

    let (mut a, mut b, mut c, mut d) = (A0, B0, C0, D0);

    for chunk_index in 0..chunks_count {
        let mut message_chunk: [u8; 64] = [0; 64];
        let message_start_index = chunk_index * 64;
        let chunk_content_length = cmp::min(message_length - message_start_index, 64);
        for i in 0..chunk_content_length {
            message_chunk[i] = message[message_start_index + i];
        }
        if (chunk_content_length < 64) {
            message_chunk[chunk_content_length] = 0x80;
        }
        if (chunk_content_length + 1 + 8 < 64) {
            let bit_length = message_length << 3;
            for i in 0..7 {
                message_chunk[56 + i] = ((bit_length >> i * 8) & 0xFF).try_into().unwrap();
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

        let (a2, b2, c2, d2) = md5_round(message_chunk_u32, a, b, c, d);
        (a, b, c, d) = (
            a.wrapping_add(a2),
            b.wrapping_add(b2),
            c.wrapping_add(c2),
            d.wrapping_add(d2),
        );
    }

    let ret: [u8; 16] = [
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
    ];
    ret
}
