use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rustc-link-lib=static=b1sdk");
    println!("cargo:rustc-link-search=native=./sdk");

    // 获取输出目录环境变量
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let profile = std::env::var("PROFILE").unwrap();

    // 设置源 DLL 路径和目标路径
    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .unwrap()
        .to_path_buf();

    let dll_name = "wukong_minimap.dll"; // 替换为你的 DLL 名称
    let source_path =
        Path::new(r"C:\Users\jaskang\code\wukong-minimap\target\release").join(dll_name);
    let dest_path =
        Path::new(r"D:\SteamLibrary\steamapps\common\BlackMythWukong\b1\Binaries\Win64")
            .join(dll_name);

    // 在构建完成后复制文件
    println!("cargo:rerun-if-changed={}", source_path.display());

    if source_path.exists() {
        match fs::copy(&source_path, &dest_path) {
            Ok(_) => println!("Successfully copied DLL to game directory"),
            Err(e) => println!("Failed to copy DLL: {}", e),
        }
    } else {
        println!("Source DLL not found at: {}", source_path.display());
    }
}
