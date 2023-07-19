fn main()
{
    cxx_build::bridge("src/lib.rs")
        .compile("ffi_otel_from_rust");

    println!("cargo:rerun-if-changed=src/lib.rs");
}