use std::{collections::HashMap, sync::Mutex};

use crate::{
    utils::{
        image_with_bytes, image_with_file, is_in_map, load_data, load_points, MapInfo, Point, Pos2,
    },
    wukong::{self, GameState},
};
use gilrs::{GamepadId, Gilrs};
use hudhook::imgui::Key;
use hudhook::{
    imgui::{self, Condition, Context, WindowFlags},
    tracing::debug,
    ImguiRenderLoop, RenderContext,
};
use image::{EncodableLayout, ImageFormat, RgbaImage};

pub struct ImageTexture {
    pub id: Option<imgui::TextureId>,
    pub image: RgbaImage,
}

impl ImageTexture {
    pub fn with_bytes(types: &[u8], format: ImageFormat) -> Self {
        Self {
            id: None,
            image: image_with_bytes(types, format),
        }
    }
    pub fn _with_file(file: &str) -> Self {
        Self {
            id: None,
            image: image_with_file(file),
        }
    }
}

struct Textures {
    pub map: ImageTexture,
    pub logo: ImageTexture,
    pub mapplayer: ImageTexture,
    pub mapwraper: ImageTexture,
    pub mainwraper: ImageTexture,

    // const categories = [
    //     "yaocai",
    //     "pass-route",
    //     "start",
    //     "yaojin",
    //     "renwu",
    //     "lingyun",
    //     "zhixian",
    //     "end",
    //     "hidden",
    //     "comment",
    // ];
    pub teleport: ImageTexture,
    pub fan: ImageTexture,          // 起点-招魂幡
    pub boss: ImageTexture,         // boss
    pub toumu: ImageTexture,        // 头目
    pub hulu: ImageTexture,         // 葫芦
    pub jiushi: ImageTexture,       // 酒食
    pub xiandan: ImageTexture,      // 仙丹
    pub baoxiang: ImageTexture,     // 宝箱
    pub zhenwan: ImageTexture,      // 珍玩
    pub fabao: ImageTexture,        // 法宝
    pub dazuo: ImageTexture,        // 打坐点
    pub cailiao: ImageTexture,      // 材料
    pub jingpo: ImageTexture,       // 精魄
    pub sandongchong: ImageTexture, // 三冬虫
    pub luojia: ImageTexture,       // 洛珈
    pub bianhua: ImageTexture,      // 变化
    pub yaojin: ImageTexture,       // 要紧事物
}

// 定义宏来简化纹理创建，仅适用于PNG格式
macro_rules! png_texture {
    ($file:expr) => {
        ImageTexture::with_bytes(include_bytes!($file), ImageFormat::Png)
    };
}

pub enum MiniMapMode {
    North, // 北边为上
    Face,  // 玩家角色面对方向为上
}
pub struct MiniMap {
    gilrs: Mutex<Gilrs>,
    current_gamepad: Option<GamepadId>,
    textures: Textures,
    map_images: HashMap<String, RgbaImage>,
    zoom: f32,
    size: f32,
    map: Option<MapInfo>,
    maps: Vec<MapInfo>,
    points: HashMap<String, Vec<Point>>,
    game: GameState,
    mode: MiniMapMode,
    is_show_main: bool,
    is_show: bool,
}

impl MiniMap {
    pub fn new() -> Self {
        wukong::init();

        let maps: Vec<MapInfo> = load_data();
        let points: HashMap<String, Vec<Point>> = load_points();

        let textures = Textures {
            map: ImageTexture::with_bytes(
                include_bytes!("../includes/nomap.webp"),
                ImageFormat::WebP,
            ),
            logo: png_texture!("../includes/logo.png"),
            mapwraper: png_texture!("../includes/mapwraper.png"),
            mainwraper: png_texture!("../includes/mainwraper.png"),
            mapplayer: png_texture!("../includes/icon_player.png"),

            teleport: png_texture!("../includes/icon_teleport.png"),
            fan: png_texture!("../includes/icon_fan.png"),
            boss: png_texture!("../includes/icon_boss.png"),
            toumu: png_texture!("../includes/icon_toumu.png"),
            hulu: png_texture!("../includes/icon_hulu.png"),
            jiushi: png_texture!("../includes/icon_jiushi.png"),
            xiandan: png_texture!("../includes/icon_xiandan.png"),
            baoxiang: png_texture!("../includes/icon_baoxiang.png"),
            zhenwan: png_texture!("../includes/icon_zhenwan.png"),
            fabao: png_texture!("../includes/icon_fabao.png"),
            dazuo: png_texture!("../includes/icon_dazuo.png"),
            yaojin: png_texture!("../includes/icon_yaojin.png"),
            cailiao: png_texture!("../includes/icon_cailiao.png"),
            jingpo: png_texture!("../includes/icon_jingpo.png"),
            sandongchong: png_texture!("../includes/icon_sandongchong.png"),
            luojia: png_texture!("../includes/icon_luojia.png"),
            bianhua: png_texture!("../includes/icon_bianhua.png"),
        };

        let mut map_images = HashMap::new();
        // 加载地图图片
        maps.iter().for_each(|map| {
            map_images.insert(map.key.clone(), image_with_file(map.key.as_str()));
        });

        let gilrs = Gilrs::new().unwrap();
        Self {
            gilrs: Mutex::new(gilrs),
            current_gamepad: None,
            textures,
            map_images,
            zoom: 0.15,
            size: 0.2,
            map: None,
            maps,
            points,
            game: wukong::game_state(),
            mode: MiniMapMode::North,
            is_show_main: false,
            is_show: true,
        }
    }

    fn update_map(&mut self) -> Option<MapInfo> {
        self.game = wukong::game_state();
        let map = self
            .maps
            .iter()
            .rev() // 从后面开始查找
            .find(|map| {
                if self.game.level == map.level
                    && self.game.x >= map.range.start[0]
                    && self.game.x <= map.range.end[0]
                    && self.game.y >= map.range.start[1]
                    && self.game.y <= map.range.end[1]
                    && self.game.z >= map.range.start[2]
                    && self.game.z <= map.range.end[2]
                {
                    return if map.areas.is_empty() {
                        true
                    } else {
                        map.areas.iter().any(|area| {
                            self.game.x >= area.start[0]
                                && self.game.x <= area.end[0]
                                && self.game.y >= area.start[1]
                                && self.game.y <= area.end[1]
                                && self.game.z >= area.start[2]
                                && self.game.z <= area.end[2]
                        })
                    };
                }
                false
            })
            .cloned();

        match (self.map.as_ref(), map) {
            (_, None) => {
                return None;
            }
            (None, Some(new_map)) => {
                return Some(new_map);
            }
            (Some(current_map), Some(new_map)) => {
                if current_map.key != new_map.key {
                    return Some(new_map);
                } else {
                    return None;
                }
            }
        }
    }

    /**
     * 将游戏坐标转换为小地图位置 0 - 1
     * flag_start是游戏地图的边界左上角，flag_end 是右下角
     * 0,0 是小地图的左上角 1,1 是小地图的右下角
     */
    fn location_to_map(&self, pos: Pos2) -> Pos2 {
        if self.map.is_none() {
            return Pos2::new(0.5, 0.5);
        }
        let map = self.map.as_ref().unwrap();
        let [x_start, y_start, _] = map.range.start;
        let [x_end, y_end, _] = map.range.end;
        let x_offset = (pos.x - x_start) / (x_end - x_start);
        let y_offset = (pos.y - y_start) / (y_end - y_start);
        Pos2::new(x_offset, y_offset)
    }
    /**
     * 将游戏中玩家转换为小地图uv
     * 玩家位于小地图的中心，根据 zoom 计算出小地图的uv
     * size 小地图可视区域的比例    
     */
    fn get_map_uv(&self, pos: Pos2) -> [[f32; 2]; 2] {
        let map_pos = self.location_to_map(pos);
        let half_zoom = self.zoom / 2.0;
        let min_pos: [f32; 2] = [map_pos.x - half_zoom, map_pos.y - half_zoom];
        let max_pos: [f32; 2] = [map_pos.x + half_zoom, map_pos.y + half_zoom];
        [[min_pos[0], min_pos[1]], [max_pos[0], max_pos[1]]]
    }

    fn get_icon_pos(&self, pos: Pos2, player_pos: Pos2, center: Pos2, view_size: f32) -> Pos2 {
        let [x_start, y_start, _] = self.map.as_ref().unwrap().range.start;
        let [x_end, y_end, _] = self.map.as_ref().unwrap().range.end;
        let x_scale = view_size / ((x_end - x_start).abs() * self.zoom);
        let y_scale = view_size / ((y_end - y_start).abs() * self.zoom);
        let pos = Pos2::new(
            center.x + (pos.x - player_pos.x) * x_scale,
            center.y + (pos.y - player_pos.y) * y_scale,
        );
        pos
    }
    fn arrow_to_p4(&self, location: Pos2, angle: f32, icon_size: f32) -> [[f32; 2]; 4] {
        let rad = angle.to_radians();
        let (sin, cos) = rad.sin_cos();
        let half = icon_size / 2.0;
        // 使用一个简单的变换矩阵计算四个角点
        let transform = |dx: f32, dy: f32| {
            [
                location.x + dx * cos - dy * sin,
                location.y + dx * sin + dy * cos,
            ]
        };
        [
            transform(-half, -half), // 左上
            transform(half, -half),  // 右上
            transform(half, half),   // 右下
            transform(-half, half),  // 左下
        ]
    }
    fn get_icon_pos_main(&self, pos: Pos2, center: Pos2, view_size: f32) -> Pos2 {
        let [x_start, y_start, _] = self.map.as_ref().unwrap().range.start;
        let [x_end, y_end, _] = self.map.as_ref().unwrap().range.end;
        let x_scale = view_size / ((x_end - x_start).abs());
        let y_scale = view_size / ((y_end - y_start).abs());
        let offset_x = center.x - view_size / 2.0;
        let offset_y = center.y - view_size / 2.0;
        Pos2::new(
            offset_x + (pos.x - x_start) * x_scale,
            offset_y + (pos.y - y_start) * y_scale,
        )
    }
    fn render(&mut self, ui: &imgui::Ui) {
        if ui.is_key_pressed_no_repeat(Key::Minus) && !ui.is_key_down(Key::LeftShift) {
            self.size = (self.size - 0.05).max(0.15);
            println!("size: {}", self.size);
        }
        if ui.is_key_pressed_no_repeat(Key::Equal) && !ui.is_key_down(Key::LeftShift) {
            self.size = (self.size + 0.05).min(0.5);
            println!("size: {}", self.size);
        }
        if ui.is_key_pressed_no_repeat(Key::Minus) && ui.is_key_down(Key::LeftShift) {
            self.zoom = (self.zoom - 0.05).max(0.15);
            println!("zoom: {}", self.zoom);
        }
        if ui.is_key_pressed_no_repeat(Key::Equal) && ui.is_key_down(Key::LeftShift) {
            self.zoom = (self.zoom + 0.05).min(0.5);
            println!("zoom: {}", self.zoom);
        }
        if ui.is_key_pressed_no_repeat(Key::Alpha0) {
            self.is_show = !self.is_show;
        }
        if ui.is_key_pressed_no_repeat(Key::Tab) {
            self.is_show_main = !self.is_show_main;
            wukong::toggle_mouse_cursor(self.is_show_main);
        }
        if let Ok(mut gilrs) = self.gilrs.lock() {
            // Examine new events
            while let Some(gilrs::Event { id, event, .. }) = gilrs.next_event() {
                self.current_gamepad = Some(id);
                println!("gilrs event from {}: {:?}", id, event);
                if let gilrs::EventType::ButtonPressed(button, code) = event {
                    let gamepad = gilrs.gamepad(id);
                    if gamepad.is_pressed(gilrs::Button::RightTrigger) {
                        match button {
                            gilrs::Button::South => {
                                self.size = (self.size - 0.05).max(0.15);
                                println!("Button South is pressed");
                            }
                            gilrs::Button::East => {
                                self.size = (self.size + 0.05).min(0.5);
                                println!("Button East is pressed");
                            }
                            gilrs::Button::DPadDown => {
                                self.is_show_main = !self.is_show_main;
                                wukong::toggle_mouse_cursor(self.is_show_main);
                                println!("Button West is pressed");
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        if self.game.playing {
            let [screen_width, screen_height] = ui.io().display_size;

            if self.is_show_main {
                let window_size = screen_width.min(screen_height);
                let map_size: f32 = window_size * 0.95;
                let icon_size = screen_width.min(screen_height) * 0.03;
                let icon_size_half = icon_size / 2.0;
                let [offset_x, offset_y] = [
                    (screen_width - window_size) / 2.0,
                    (screen_height - window_size) / 2.0,
                ];
                let center = Pos2::new(offset_x + window_size / 2.0, offset_y + window_size / 2.0);

                ui.window("wukong-mainmap")
                    .size([window_size, window_size], Condition::Always)
                    .position([offset_x, offset_y], Condition::Always)
                    .flags(
                        WindowFlags::NO_DECORATION
                            | WindowFlags::NO_MOVE
                            | WindowFlags::NO_INPUTS
                            | WindowFlags::NO_NAV
                            | WindowFlags::NO_BACKGROUND,
                    )
                    .build(|| {
                        ui.set_cursor_pos([0.0, 0.0]);
                        let draw_list = ui.get_window_draw_list();

                        if let Some(map) = self.map.as_ref() {
                            // 绘制地图
                            let map_image = self.textures.map.id.unwrap();
                            let map_offset_x = offset_x + (window_size - map_size) / 2.0;
                            let map_offset_y = offset_y + (window_size - map_size) / 2.0;

                            draw_list
                                .add_image_rounded(
                                    map_image,
                                    [map_offset_x, map_offset_y],
                                    [map_offset_x + map_size, map_offset_y + map_size],
                                    0.0,
                                )
                                .build();

                            // 绘制地图图标
                            self.points
                                .get(map.level.as_str())
                                .unwrap_or(&vec![])
                                .iter()
                                .filter(|point| is_in_map(point, &map))
                                .for_each(|point| {
                                    let icon = match point.category.as_str() {
                                        "teleport" => self.textures.teleport.id,
                                        "fan" => self.textures.fan.id,
                                        "boss" => self.textures.boss.id,
                                        "toumu" => self.textures.toumu.id,
                                        "hulu" => self.textures.hulu.id,
                                        "jiushi" => self.textures.jiushi.id,
                                        "xiandan" => self.textures.xiandan.id,
                                        "baoxiang" => self.textures.baoxiang.id,
                                        "zhenwan" => self.textures.zhenwan.id,
                                        "fabao" => self.textures.fabao.id,
                                        "dazuo" => self.textures.dazuo.id,
                                        "cailiao" => self.textures.cailiao.id,
                                        "jingpo" => self.textures.jingpo.id,
                                        "sandongchong" => self.textures.sandongchong.id,
                                        "luojia" => self.textures.luojia.id,
                                        "bianhua" => self.textures.bianhua.id,
                                        "yaojin" => self.textures.yaojin.id,
                                        _ => None,
                                    };

                                    if let Some(id) = icon {
                                        let icon_pos = self.get_icon_pos_main(
                                            Pos2::new(point.x, point.y),
                                            center,
                                            map_size,
                                        );
                                        draw_list
                                            .add_image(
                                                id,
                                                [
                                                    icon_pos.x - icon_size_half,
                                                    icon_pos.y - icon_size_half,
                                                ],
                                                [
                                                    icon_pos.x + icon_size_half,
                                                    icon_pos.y + icon_size_half,
                                                ],
                                            )
                                            .build();
                                    }
                                });

                            let player_pos = self.get_icon_pos_main(
                                Pos2::new(self.game.x, self.game.y),
                                center,
                                map_size,
                            );
                            // 绘制玩家角色箭头
                            let [p0, p1, p2, p3] =
                                self.arrow_to_p4(player_pos, self.game.angle, icon_size);
                            draw_list
                                .add_image_quad(self.textures.mapplayer.id.unwrap(), p0, p1, p2, p3)
                                .build();

                            // 绘制外围地图边框
                            draw_list
                                .add_image(
                                    self.textures.mainwraper.id.unwrap(),
                                    [offset_x, offset_y],
                                    [offset_x + window_size, offset_y + window_size],
                                )
                                .build();
                        } else {
                            debug!("draw_nomap");
                        }
                    });
            }
            if self.is_show {
                let window_size = screen_width.min(screen_height) * self.size;
                let map_size: f32 = window_size * 0.947;
                let map_size_half = map_size / 2.0;
                let icon_size = screen_width.min(screen_height) * 0.03;
                let icon_size_half = icon_size / 2.0;
                let [offset_x, offset_y] = [screen_width - window_size - 10.0, 10.0];
                let center = Pos2::new(offset_x + window_size / 2.0, offset_y + window_size / 2.0);

                ui.window("wukong-minimap")
                    .size([window_size, window_size], Condition::Always)
                    .position([offset_x, offset_y], Condition::Always)
                    .flags(
                        WindowFlags::NO_DECORATION
                            | WindowFlags::NO_MOVE
                            | WindowFlags::NO_INPUTS
                            | WindowFlags::NO_NAV
                            | WindowFlags::NO_BACKGROUND,
                    )
                    .build(|| {
                        ui.set_cursor_pos([0.0, 0.0]);
                        let draw_list = ui.get_window_draw_list();

                        if let Some(map) = self.map.as_ref() {
                            // 绘制地图
                            let map_image = self.textures.map.id.unwrap();
                            let map_offset_x = offset_x + (window_size - map_size) / 2.0;
                            let map_offset_y = offset_y + (window_size - map_size) / 2.0;
                            let map_uv = self.get_map_uv(Pos2::new(self.game.x, self.game.y));

                            draw_list
                                .add_image_rounded(
                                    map_image,
                                    [map_offset_x, map_offset_y],
                                    [map_offset_x + map_size, map_offset_y + map_size],
                                    map_size / 2.0,
                                )
                                .uv_min(map_uv[0])
                                .uv_max(map_uv[1])
                                .build();

                            // 绘制地图图标
                            self.points
                                .get(map.level.as_str())
                                .unwrap_or(&vec![])
                                .iter()
                                .filter(|point| is_in_map(point, &map))
                                .for_each(|point| {
                                    let icon_pos = self.get_icon_pos(
                                        Pos2::new(point.x, point.y),
                                        Pos2::new(self.game.x, self.game.y),
                                        center,
                                        map_size,
                                    );
                                    let icon = match point.category.as_str() {
                                        "teleport" => self.textures.teleport.id,
                                        "fan" => self.textures.fan.id,
                                        "boss" => self.textures.boss.id,
                                        "toumu" => self.textures.toumu.id,
                                        "hulu" => self.textures.hulu.id,
                                        "jiushi" => self.textures.jiushi.id,
                                        "xiandan" => self.textures.xiandan.id,
                                        "baoxiang" => self.textures.baoxiang.id,
                                        "zhenwan" => self.textures.zhenwan.id,
                                        "fabao" => self.textures.fabao.id,
                                        "dazuo" => self.textures.dazuo.id,
                                        "cailiao" => self.textures.cailiao.id,
                                        "jingpo" => self.textures.jingpo.id,
                                        "sandongchong" => self.textures.sandongchong.id,
                                        "luojia" => self.textures.luojia.id,
                                        "bianhua" => self.textures.bianhua.id,
                                        "yaojin" => self.textures.yaojin.id,
                                        _ => None,
                                    };

                                    if let Some(id) = icon {
                                        // 判断是否在可视区域内, icon_pos 和 center 之间的距离小于 map_size / 2 - icon_size_half
                                        let distance = ((icon_pos.x - center.x).powi(2)
                                            + (icon_pos.y - center.y).powi(2))
                                        .sqrt();
                                        if distance <= map_size_half - icon_size_half {
                                            draw_list
                                                .add_image(
                                                    id,
                                                    [
                                                        icon_pos.x - icon_size_half,
                                                        icon_pos.y - icon_size_half,
                                                    ],
                                                    [
                                                        icon_pos.x + icon_size_half,
                                                        icon_pos.y + icon_size_half,
                                                    ],
                                                )
                                                .build();
                                        }
                                    }
                                });

                            // 绘制玩家角色箭头
                            let [p0, p1, p2, p3] =
                                self.arrow_to_p4(center, self.game.angle, icon_size);
                            draw_list
                                .add_image_quad(self.textures.mapplayer.id.unwrap(), p0, p1, p2, p3)
                                .build();

                            // 绘制外围地图边框
                            draw_list
                                .add_image(
                                    self.textures.mapwraper.id.unwrap(),
                                    [offset_x, offset_y],
                                    [offset_x + window_size, offset_y + window_size],
                                )
                                .build();
                        } else {
                            debug!("draw_nomap");
                        }
                    });

                let logo_width = screen_width.min(screen_height) * 0.065;
                let logo_height = logo_width * 0.28;

                ui.window("logo")
                    .position(
                        [offset_x, offset_y + window_size + logo_height / 4.0],
                        imgui::Condition::Always,
                    )
                    .size([window_size, logo_height], imgui::Condition::Always)
                    .flags(
                        WindowFlags::NO_DECORATION
                            | WindowFlags::NO_MOVE
                            | WindowFlags::NO_INPUTS
                            | WindowFlags::NO_NAV
                            | WindowFlags::NO_BACKGROUND,
                    )
                    .build(|| {
                        let draw_list = ui.get_window_draw_list();
                        let logo_offset_x = offset_x + (window_size - logo_width) / 2.0;
                        let logo_offset_y = offset_y + window_size + logo_height / 4.0;

                        draw_list
                            .add_image(
                                self.textures.logo.id.unwrap(),
                                [logo_offset_x, logo_offset_y],
                                [logo_offset_x + logo_width, logo_offset_y + logo_height],
                            )
                            .build();
                        // Image::new(tex_id, [logo_width, logo_height])
                        //     .uv0([0.0, 0.0])
                        //     .uv1([1.0, 1.0])
                        //     .build(ui);
                    });
            }
        }
    }
}

impl ImguiRenderLoop for MiniMap {
    fn initialize<'a>(&'a mut self, ctx: &mut Context, render_context: &'a mut dyn RenderContext) {
        let io = ctx.io_mut();
        io.mouse_draw_cursor = false;
        let style = ctx.style_mut();
        style.window_rounding = 10.0;
        style.window_padding = [0.0, 0.0];
        // text red
        // style.colors[imgui::StyleColor::Text as usize] = [0.0, 0.0, 1.0, 1.0];

        // 定义宏来简化纹理加载
        macro_rules! load_textures {
            ($($texture:ident),*) => {
                $(
                    self.textures.$texture.id = Some(
                        render_context
                            .load_texture(
                                self.textures.$texture.image.as_bytes(),
                                self.textures.$texture.image.width(),
                                self.textures.$texture.image.height(),
                            )
                            .unwrap(),
                    );
                )*
            }
        }

        // 使用宏加载所有纹理
        load_textures!(
            map,
            mapplayer,
            mapwraper,
            mainwraper,
            logo,
            teleport,
            fan,
            boss,
            toumu,
            hulu,
            jiushi,
            xiandan,
            baoxiang,
            zhenwan,
            fabao,
            dazuo,
            cailiao,
            jingpo,
            sandongchong,
            luojia,
            bianhua,
            yaojin
        );
    }
    fn before_render<'a>(
        &'a mut self,
        _ctx: &mut Context,
        render_context: &'a mut dyn RenderContext,
    ) {
        let map = self.update_map();
        if let Some(map) = map {
            println!("update map: {:?}", map);
            let map_image = self.map_images.get(map.key.as_str());
            if let Some(map_image) = map_image {
                let _ = render_context.replace_texture(
                    self.textures.map.id.unwrap(),
                    map_image.as_bytes(),
                    map_image.width(),
                    map_image.height(),
                );
            }
            self.map = Some(map);
        }
    }
    fn render(&mut self, ui: &mut imgui::Ui) {
        self.render(ui);
        // ui.show_demo_window(&mut true);
    }
}
