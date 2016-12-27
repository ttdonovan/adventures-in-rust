# RustBridge

* https://github.com/rust-community/rustbridge

## Troubleshooting OpenSSL

```
  $ brew install openssl
  $ export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include
  $ export DEP_OPENSSL_INCLUDE=/usr/local/opt/openssl/include
  $ cargo clean
  $ cargo build
```
