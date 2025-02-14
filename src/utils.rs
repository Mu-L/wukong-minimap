use image::{ImageFormat, ImageReader, RgbaImage};
use serde::{Deserialize, Serialize};
use std::{io::Cursor, path::PathBuf};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Pos2 {
    pub x: f32,
    pub y: f32,
}

impl Pos2 {
    pub fn new(x: f32, y: f32) -> Self {
        Pos2 { x, y }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Point {
    pub name: String,
    pub key: String,
    pub icon: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Area {
    pub start: [f32; 3],
    pub end: [f32; 3],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapInfo {
    pub key: String,
    pub level: String,
    pub range: Area,
    pub scale: f32,
    pub areas: Vec<Area>,
    pub url: String,
    pub points: Vec<Point>,
}

pub fn get_dll_dir() -> PathBuf {
    let dll_path = hudhook::util::get_dll_path().unwrap();
    dll_path.parent().unwrap().to_path_buf()
}

pub fn load_data() -> Vec<MapInfo> {
    let dll_dir = get_dll_dir();
    let maps_dir = dll_dir.join("maps");
    if !maps_dir.exists() {
        std::fs::create_dir_all(maps_dir.clone()).unwrap();
    }

    let data = map_data();

    // for map in data.clone() {
    //     let file_path = maps_dir.join(&map.key);
    //     if !file_path.exists() {
    //         let image = blocking::get(&map.url).unwrap().bytes().unwrap();
    //         std::fs::write(&file_path, &image).unwrap();
    //     }
    // }
    data
}

pub fn setup_tracing() {
    std::env::set_var("RUST_BACKTRACE", "full");
    std::env::set_var("RUST_LOG", "trace");
    hudhook::alloc_console().unwrap();
    hudhook::enable_console_colors();
}

pub fn image_with_bytes(bytes: &[u8], format: ImageFormat) -> RgbaImage {
    ImageReader::with_format(Cursor::new(bytes), format)
        .decode()
        .unwrap()
        .into_rgba8()
}

pub fn image_with_file(file: &str) -> RgbaImage {
    let dll_dir = get_dll_dir();
    let file_path = dll_dir.join("maps").join(file);
    let image = ImageReader::open(&file_path)
        .unwrap_or_else(|_| panic!("Failed to open image: {}", file_path.display()))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .into_rgba8();
    image
}

pub fn map_data() -> Vec<MapInfo> {
    let data = include_str!("./../includes/data.json");
    let maps: Vec<MapInfo> = serde_json::from_str(data).unwrap();
    maps
}
