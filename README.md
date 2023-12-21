Example for [cxx](https://github.com/dtolnay/cxx) with `RUST_CXX_NO_EXCEPTIONS` flag.

Build with exceptions **enabled**:

```sh
cargo build && ./build-cpp.sh
```

Build with exceptions **disabled**:

```sh
cargo build --features no-exceptions && ./build-cpp_no-exceptions.sh
```