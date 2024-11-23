use winres::WindowsResource;

fn main() {
    let res = WindowsResource::new();

    println!("cargo:rerun-if-changed=interceptor.asm");
    println!("cargo:rustc-env=CARGO_CFG_TARGET_FEATURE=fxsr,sse,sse2,avx");

    cc::Build::new()
        .file("src/interceptor.asm")
        .compile("interceptor");

    res.compile().unwrap();
}
