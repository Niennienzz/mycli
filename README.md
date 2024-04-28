# GeekTime Rust Camp Assignment #01

## Requirements

Write a Rust CLI program that supports the following three subcommands.

### 01 | Text Encryption

- Encrypt the input text using the [ChaCha20-Poly1305](https://en.wikipedia.org/wiki/ChaCha20-Poly1305) algorithm.
- The input text can be read from a file or from the command line.
- The output is a base64-encoded string written to the console.

```bash
mycli text encrypt --key XXX --input hello.txt
mycli text decrypt --key XXX --input encrypted_hello.txt
```

### 02 | JWT Generation and Verification

- Generate JWT tokens with the following claims: `sub`, `aud`, `exp`.
- Generated JWT tokens should pass the verification on [jwt.io](https://jwt.io/).

```bash
mycli jwt sign --sub acme --aud device1 --exp 14d
mycli jwt verify --token "{token}"
```

### 03 | HTTP Server

- Add directory index support to the HTTP server from the course.
