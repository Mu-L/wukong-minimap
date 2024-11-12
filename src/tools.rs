use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;

use image::{ImageReader, RgbaImage};

pub fn load_image(file: &str) -> RgbaImage {
    let dll_path = hudhook::util::get_dll_path().unwrap();
    // 获取DLL的目录
    let file_path = dll_path.parent().unwrap().join("assets").join(file);

    let image = ImageReader::open(&file_path)
        .unwrap_or_else(|_| panic!("Failed to open image: {}", file_path.display()))
        .with_guessed_format()
        .unwrap_or_else(|_| panic!("Failed to guess image format: {}", file_path.display()))
        .decode()
        .unwrap_or_else(|_| panic!("Failed to decode image: {}", file_path.display()))
        .into_rgba8();
    image
}

pub fn load_json<T>(file: &str) -> Vec<T>
where
    T: DeserializeOwned,
{
    let dll_path = hudhook::util::get_dll_path().unwrap();
    // 获取DLL的目录
    let file_path = dll_path.parent().unwrap().join("assets").join(file);

    let file = File::open(file_path);
    if let Ok(file) = file {
        let file = BufReader::new(file);
        let ret: Vec<T> = serde_json::from_reader(file).unwrap();
        return ret;
    }
    vec![]
}

// 实现一个函数来判断点的位置
pub fn point_inside(p1: [f32; 2], p2: [f32; 2], p: [f32; 2]) -> bool {
    // 计算直线的方向向量
    let direction = [p2[0] - p1[0], p2[1] - p1[1]];
    // 计算点到直线起点的向量
    let vector = [p[0] - p1[0], p[1] - p1[1]];
    // 计算两个向量的叉积
    let cross_product = direction[0] * vector[1] - direction[1] * vector[0];
    // 根据叉积的符号判断点的位置
    cross_product >= 0.0
}
#[macro_export]
macro_rules! include_image {
    ($path:expr) => {{
        use image::ImageReader;
        use std::io::Cursor;

        ImageReader::new(Cursor::new(include_bytes!($path)))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
            .into_rgba8()
    }};
}
