fn main() {
    println!("cargo:rustc-link-lib=static=b1sdk");
    println!("cargo:rustc-link-search=native=./sdk");
}
