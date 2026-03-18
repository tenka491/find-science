use base64::{engine::general_purpose, Engine};

pub fn encode_image(bytes: &[u8]) -> String {
    general_purpose::STANDARD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_image() {
        let data = b"hello";
        let encoded = encode_image(data);

        assert_eq!(encoded, "aGVsbG8=");
    }
}