pub struct Game {
    pub enable: bool,
    pub open: bool,
    pub map_id: i32,
    pub status: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub angle: f32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            enable: false,
            open: false,
            map_id: 0,
            status: false,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            angle: 0.0,
        }
    }
    pub fn update(&mut self, map_id: i32, status: bool, x: f32, y: f32, z: f32, angle: f32) {
        self.map_id = map_id;
        self.status = status;
        self.x = x;
        self.y = y;
        self.z = z;
        self.angle = angle;
    }
    pub fn toggle(&mut self) {
        self.enable = !self.enable;
    }
    pub fn toggle_big_map(&mut self) {
        self.open = !self.open;
    }
}
