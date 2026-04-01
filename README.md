# SHA-256 from scratch in Rust

Hey! This is my implementation of SHA-256, written completely from scratch in Rust.

I didn't use any libraries or copy anyone's code — I just read the math behind the algorithm ([FIPS 180-4](https://csrc.nist.gov/publications/detail/fips/180/4/final)) and tried to turn it into working code. This is one of my first projects on GitHub so please bear with me :)

## What is this

SHA-256 is a cryptographic hash function. You give it any data, and it returns a unique 256-bit (32 byte) fingerprint. It's used everywhere in crypto/web3, Bitcoin mining, digital signatures, etc.

I wanted to actually understand how it works instead of just calling a library, so I built it myself.

## What I implemented

- Padding (extending input to 512-bit blocks)
- Message schedule (expanding 16 words into 64)
- Compression function (64 rounds with bitwise magic)
- Davies-Meyer feedforward
- All the bit rotation and XOR stuff that makes SHA-256 work

No dependencies, just standard Rust.

## How to run

```bash
cargo run
```

It will ask you to type something, then print the hash:

```
Enter text to hash:
hello
hash: 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
```

## Does it actually work?

Yes! I tested it against official NIST test vectors:

```bash
cargo test
```

All 6 tests pass, including the big one (hashing 1 million 'a' characters):

- ✅ empty string
- ✅ "abc"
- ✅ "a"
- ✅ "hello world"
- ✅ 448-bit two-block message
- ✅ 1,000,000 x 'a' (official NIST long-message test)

## Files

```
src/
  main.rs      — entry point, reads input from terminal
  sha256.rs    — the actual SHA-256 implementation
tests/
  nist_vectors.rs — test cases from the NIST standard
```

## Built with

- Rust
- Math from FIPS 180-4
- Lots of debugging :)
