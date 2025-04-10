use hudhook::tracing;
use image::{ImageFormat, ImageReader, RgbaImage};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::sync::Mutex;
use std::{collections::HashMap, io::Cursor, path::PathBuf};
use time::{macros::format_description, UtcOffset};
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

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
    // pub key: String,
    pub category: String,
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
}

pub fn get_dll_dir() -> PathBuf {
    let dll_path = hudhook::util::get_dll_path().unwrap();
    dll_path.parent().unwrap().to_path_buf()
}

pub fn setup_tracing() {
    // hudhook::alloc_console().unwrap();
    // hudhook::enable_console_colors();
    // dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");

    let log_file = hudhook::util::get_dll_path()
        .map(|mut path| {
            path.set_extension("log");
            path
        })
        .and_then(|path| File::create(path).ok())
        .unwrap();

    // 配置日志时间格式
    // 配置时区为东8区，
    let offset = UtcOffset::from_hms(8, 0, 0).unwrap_or(UtcOffset::UTC);
    // 时间格式为  年-月-日 时:分:秒 格式
    let logger_time = OffsetTime::new(
        offset,
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    );

    tracing_subscriber::registry()
        .with(
            fmt::layer().event_format(
                fmt::format()
                    .with_timer(logger_time.clone()) //打印时间
                    .with_level(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_thread_names(true),
            ),
        )
        .with(
            fmt::layer()
                .with_timer(logger_time.clone()) //打印时间
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

pub fn load_data() -> Vec<MapInfo> {
    let data = include_str!("./../includes/data.json");
    let maps: Vec<MapInfo> = serde_json::from_str(data).unwrap();
    maps
}

pub fn load_points() -> HashMap<String, Vec<Point>> {
    let data = include_str!("./../includes/data_points.json");
    let points: HashMap<String, Vec<Point>> = serde_json::from_str(data).unwrap();
    points
}

pub fn is_in_range(point: &Point, range: &Area) -> bool {
    point.x >= range.start[0]
        && point.x <= range.end[0]
        && point.y >= range.start[1]
        && point.y <= range.end[1]
        && point.z >= range.start[2]
        && point.z <= range.end[2]
}

pub fn is_in_map(point: &Point, map: &MapInfo) -> bool {
    let in_range = is_in_range(point, &map.range);
    // 判断是否在地图的区域范围内, 如果地图有区域, 需要至少一个区域包含该点
    if map.areas.is_empty() {
        in_range
    } else {
        map.areas.iter().any(|area| is_in_range(point, area))
    }
}
