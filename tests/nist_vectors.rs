// Official NIST FIPS 180-4 test vectors for SHA-256

mod sha256_mod {
    include!("../src/sha256.rs");
}
use sha256_mod::{Sha256, to_hex};

// NIST Example 1: "abc"
#[test]
fn nist_one_block_abc() {
    let hash = Sha256::new().finalize(b"abc");
    assert_eq!(
        to_hex(&hash),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

// NIST Example 2: Two-block message (448 bits)
#[test]
fn nist_two_block_448() {
    let hash = Sha256::new().finalize(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq");
    assert_eq!(
        to_hex(&hash),
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
    );
}

// NIST Example 3: Empty string
#[test]
fn nist_empty() {
    let hash = Sha256::new().finalize(b"");
    assert_eq!(
        to_hex(&hash),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

// Single character
#[test]
fn test_single_a() {
    let hash = Sha256::new().finalize(b"a");
    assert_eq!(
        to_hex(&hash),
        "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb"
    );
}

// Common test: "hello world"
#[test]
fn test_hello_world() {
    let hash = Sha256::new().finalize(b"hello world");
    assert_eq!(
        to_hex(&hash),
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
}

// NIST long-message test: 1,000,000 x 'a'
#[test]
fn nist_million_a() {
    let input = vec![b'a'; 1_000_000];
    let hash = Sha256::new().finalize(&input);
    assert_eq!(
        to_hex(&hash),
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
    );
}
