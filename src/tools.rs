use serde::de::DeserializeOwned;
use std::f32::consts::PI;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use image::{ImageReader, RgbaImage};
use tracing::debug;

pub fn setup_tracing() {
    std::env::set_var("RUST_LOG", "trace");
    hudhook::alloc_console().unwrap();
    hudhook::enable_console_colors();

    let log_file = hudhook::util::get_dll_path()
        .map(|mut path| {
            path.set_extension("log");
            path
        })
        .and_then(|path| File::create(path).ok())
        .unwrap();

    tracing_subscriber::registry()
        .with(
            fmt::layer().event_format(
                fmt::format()
                    .with_level(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_thread_names(true),
            ),
        )
        .with(
            fmt::layer()
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true)
                .with_thread_names(true)
                .with_writer(Mutex::new(log_file))
                .with_ansi(false)
                .boxed(),
        )
        .with(EnvFilter::from_default_env())
        .init();
}

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
    debug!("file_path: {}", &file_path.to_str().unwrap());
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

pub fn vector_to_angle(angle_x: f32, angle_y: f32) -> f32 {
    let angle_radians = angle_x.atan2(angle_y); // 使用 atan2 计算角度
    let angle_degrees = angle_radians * (360.0 / PI);
    let angle_degrees = angle_degrees + 90.0;
    if angle_degrees < 0.0 {
        angle_degrees + 360.0
    } else if angle_degrees > 360.0 {
        angle_degrees - 360.0
    } else {
        angle_degrees
    }
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
