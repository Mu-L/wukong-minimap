use std::sync::{Mutex, OnceLock};

static GAME_STATE: OnceLock<Mutex<Game>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct Game {
    pub enable: bool,
    pub open: bool,
    pub level: i32,
    pub status: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub angle: f32,
    pub prev_level: i32,
}

impl Game {
    fn new() -> Self {
        Self {
            enable: false,
            open: false,
            level: 0,
            status: false,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            angle: 0.0,
            prev_level: 0,
        }
    }

    // 如果使用 OnceLock，需要在程序启动时初始化
    pub fn init() {
        GAME_STATE.get_or_init(|| Mutex::new(Game::new()));
    }
    // 内部帮助方法，处理锁的获取
    fn with_game<F, R>(f: F) -> R
    where
        F: FnOnce(&mut Game) -> R,
    {
        let mut game = GAME_STATE.get().unwrap().lock().unwrap();
        f(&mut game)
    }

    // 写入访问的统一方法
    fn modify<F>(f: F)
    where
        F: FnOnce(&mut Game),
    {
        Self::with_game(f);
    }

    // 只读访问
    pub fn get_game() -> Game {
        GAME_STATE.get().unwrap().lock().unwrap().clone()
    }
    // 使用 modify 重写 update
    pub fn update(level: i32, status: bool, x: f32, y: f32, z: f32, angle: f32) {
        Self::modify(|game| {
            game.level = level;
            game.status = status;
            game.x = x;
            game.y = y;
            game.z = z;
            game.angle = angle;
            game.prev_level = game.level;
        });
        println!("game: {:?}", Self::get_game());
    }

    // toggle 方法保持一致
    pub fn toggle() {
        Self::modify(|game| game.enable = !game.enable);
    }

    // toggle_big_map 方法保持一致
    pub fn toggle_big_map() {
        Self::modify(|game| game.open = !game.open);
    }
}
