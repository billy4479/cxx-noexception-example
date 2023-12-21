fn main() {
    let mut build = cxx_build::bridge("src/lib.rs");

    if cfg!(features = "no-exceptions") {
        build
            .define("RUST_CXX_NO_EXCEPTIONS", None)
            .flag("-fno-exceptions");
    }

    build.compile("cxx-noexception-example");
}
