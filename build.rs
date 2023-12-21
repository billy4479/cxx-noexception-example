fn main() {
    cxx_build::bridge("src/lib.rs")
        .define("RUST_CXX_NO_EXCEPTIONS", None)
        // .flag("-fno-exceptions")
        .compile("cxx-noexception-example");
}
