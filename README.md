# GeekTime Rust Camp Assignment #01

## Requirements

- Write a Rust CLI program that supports the following three subcommands.
- Course examples can be found [here](https://github.com/tyr-rust-bootcamp/01-rcli).

### 01 | Text Encryption

- Encrypt the input text using the [ChaCha20-Poly1305](https://en.wikipedia.org/wiki/ChaCha20-Poly1305) algorithm.
- The input text can be read from a file or from the command line.
- The output is a base64-encoded string written to the console.

```bash
./mycli text encrypt --key XXXXXXXX --input hello.txt
./mycli text decrypt --key XXXXXXXX --input encrypted_hello.txt
```

### 02 | JWT Generation and Verification

- Generate JWT tokens with the following claims: `sub`, `aud`, `exp`.
- Generated JWT tokens should pass the verification on [jwt.io](https://jwt.io/).

```bash
./mycli jwt sign --key XXXXXXXX --sub user1 --aud "https://u.geekbang.org" --exp 2524626000
./mycli jwt verify --key XXXXXXXX --token eyJ0eXXXXX.eyJzdXXXXX.XXXXX
```

### 03 | HTTP Server

- Add directory index support to the HTTP server from the course.
- Note `templates` is not bundled into the binary, the following command should be run from the project root.

```bash
./mycli http serve --port 3000 --dir /path/to/dir
```

---

## How to Run

- Rust and cargo version `1.77.2+` is required.

```bash
cargo build --release --bin mycli
cd target/release
```

- Then you can use the commands mentioned above.
