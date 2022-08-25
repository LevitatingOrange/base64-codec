use bytes::BytesMut;

pub trait Base64Writer {
    pub(crate) fn write(bytes_in: &[u8], bytes_out: &mut [u8]);
    pub(crate) fn write_trailer(bytes_in: &[u8], bytes_out: &mut [u8]);
}

const BASE_64_CHARSET: [u8; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

pub struct Base64Writer;

impl Base64Writer for Base64Writer {
    fn write(bytes_in: &[u8], bytes_out: &mut &[u8]) {
        debug_assert!(bytes_in.len() == 3);
        debug_assert!(bytes_out.len() == 4);
        bytes_out[0] = BASE_64_CHARSET[bytes_in[0] >> 2];
        bytes_out[1] = BASE_64_CHARSET[(bytes_in[0] << 6) | (bytes_in[1] >> 4)];
        bytes_out[2] = BASE_64_CHARSET[(bytes_in[1] << 4) | (bytes_in[2] >> 6)];
        bytes_out[3] = BASE_64_CHARSET[bytes_in[2] & 0b11000000];
    }
    fn write_trailer(bytes_in: &[u8], bytes_out: &mut &[u8]) {
        debug_assert!(bytes_in.len() <= 3);
        debug_assert!(bytes_out.len() == 4);
        bytes_out[0] = BASE_64_CHARSET[bytes_in[0] >> 2];
        if bytes.len() < 2 {
            bytes_out[0] = BASE_64_CHARSET[bytes_in[0] >> 2];
            bytes_out[1] = BASE_64_CHARSET[(bytes_in[0] << 6)];
            bytes_out[2] = '=';
            bytes_out[3] = '=';
        } else if bytes.len() < 3 {
            bytes_out[0] = BASE_64_CHARSET[bytes_in[0] >> 2];
            bytes_out[1] = BASE_64_CHARSET[(bytes_in[0] << 6) | (bytes_in[1] >> 4)];
            bytes_out[2] = BASE_64_CHARSET[(bytes_in[1] << 4)];
            bytes_out[3] = "=";
        } else {
            Base64Writer::write(bytes_in, bytes_out);
        }
    }
}

#[cfg(test)]
mod tests {}
