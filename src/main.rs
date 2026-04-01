mod sha256;
use crate::sha256::{Sha256, to_hex};

fn main() {
    let mut data = String::new();
    println!("Enter text to hash:");
    std::io::stdin()
        .read_line(&mut data)
        .expect("Failed to read line");
    let data = data.trim();

    let hasher = Sha256::new();
    let hash = hasher.finalize(data.as_bytes());

    println!("hash: {}", to_hex(&hash));
}

#[cfg(test)]
mod tests {
    use super::sha256::{Sha256, to_hex};

    #[test]
    fn test_empty_string() {
        let hash = Sha256::new().finalize(b"");
        assert_eq!(
            to_hex(&hash),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_abc() {
        let hash = Sha256::new().finalize(b"abc");
        assert_eq!(
            to_hex(&hash),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn test_448_bits() {
        let hash = Sha256::new().finalize(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq");
        assert_eq!(
            to_hex(&hash),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
    }
}
