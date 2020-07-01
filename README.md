# Rusty Jeff

Rust library for Python to handle decryption of base64-encoded text.

## Local Usage

*Note: this needs to be expanded on*

*In terminal*

```bash
cargo build --release
ln -s target/release/librustyjeff.dylib rustyjeff.so
```

*In python*

```python
import rustyjeff

# returns a string on success
# raises a ValueError on failure containing error as a string
rustyjeff.rsa_decrypt_base64(priv_key_pem, encrypted_base64)
```

# PyPI deployment

**TODO**
