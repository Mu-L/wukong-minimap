use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

use crate::{
    utils::{image_with_bytes, image_with_file, load_data, MapInfo, Pos2},
    wukong::{self, GameState},
};
use hudhook::imgui::Key;
use hudhook::tracing;
use hudhook::{
    imgui::{self, Condition, Context, StyleVar, WindowFlags},
    tracing::debug,
    ImguiRenderLoop, RenderContext,
};
use image::{EncodableLayout, RgbaImage};

pub struct ImageTexture {
    pub id: Option<imgui::TextureId>,
    pub image: RgbaImage,
}

impl ImageTexture {
    pub fn with_bytes(types: &[u8]) -> Self {
        Self {
            id: None,
            image: image_with_bytes(types),
        }
    }
    pub fn with_file(file: &str) -> Self {
        Self {
            id: None,
            image: image_with_file(file),
        }
    }
}

struct Textures {
    pub teleport: ImageTexture,
    pub mapplayer: ImageTexture,
    pub fan: ImageTexture,

    pub map: ImageTexture,
    pub logo: ImageTexture,
}

pub struct MiniMap {
    textures: Textures,
    map_images: HashMap<String, RgbaImage>,
    zoom: f32,
    size: f32,
    map: Option<MapInfo>,
    maps: Vec<MapInfo>,
    game: GameState,
}

impl MiniMap {
    pub fn new() -> Self {
        wukong::init();

        let maps: Vec<MapInfo> = load_data();

        let textures = Textures {
            map: ImageTexture::with_bytes(include_bytes!("../includes/nomap.png")),
            teleport: ImageTexture::with_bytes(include_bytes!("../includes/teleport.png")),
            mapplayer: ImageTexture::with_bytes(include_bytes!("../includes/mapplayer.png")),
            fan: ImageTexture::with_bytes(include_bytes!("../includes/fan.png")),
            logo: ImageTexture::with_bytes(include_bytes!("../includes/logo.png")),
        };

        let mut map_images = HashMap::new();
        // 加载地图图片
        maps.iter().for_each(|map| {
            map_images.insert(map.key.clone(), image_with_file(map.key.as_str()));
        });

        Self {
            textures,
            map_images,
            zoom: 0.1,
            size: 0.15,
            map: None,
            maps,
            game: wukong::game_state(),
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

    fn get_icon_pos(&self, pos: Pos2, player_pos: Pos2, center: Pos2, view_size: f32) -> [Pos2; 2] {
        let [x_start, y_start, _] = self.map.as_ref().unwrap().range.start;
        let [x_end, y_end, _] = self.map.as_ref().unwrap().range.end;
        let x_scale = view_size / ((x_end - x_start).abs() * self.zoom);
        let y_scale = view_size / ((y_end - y_start).abs() * self.zoom);
        let pos = Pos2::new(
            center.x + (pos.x - player_pos.x) * x_scale,
            center.y + (pos.y - player_pos.y) * y_scale,
        );
        let min_pos = Pos2::new(pos.x - 16.0, pos.y - 16.0);
        let max_pos = Pos2::new(pos.x + 16.0, pos.y + 16.0);
        [min_pos, max_pos]
    }
    fn arrow_to_p4(&self, location: Pos2, size: f32, angle: f32) -> [[f32; 2]; 4] {
        let rad = angle.to_radians();
        let (sin, cos) = rad.sin_cos();
        let half = size / 2.0;
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
    fn render(&mut self, ui: &imgui::Ui) {
        if ui.is_key_pressed_no_repeat(Key::Minus) {
            self.size = (self.size - 0.05).max(0.15);
            println!("size: {}", self.size);
        }
        if ui.is_key_pressed_no_repeat(Key::Equal) {
            self.size = (self.size + 0.05).min(0.5);
            println!("size: {}", self.size);
        }
        if self.game.playing {
            let [screen_width, screen_height] = ui.io().display_size;
            let window_size = (screen_width * self.size).min(screen_height * self.size);

            let [offset_x, offset_y] = [screen_width - window_size - 10.0, 10.0];
            ui.window("wukong-minimap")
                .size([window_size, window_size], Condition::Always)
                .position([offset_x, offset_y], Condition::Always)
                .flags(
                    WindowFlags::NO_TITLE_BAR
                        | WindowFlags::NO_RESIZE
                        | WindowFlags::NO_MOVE
                        | WindowFlags::NO_SCROLLBAR
                        | WindowFlags::NO_BACKGROUND
                        | WindowFlags::NO_INPUTS
                        | WindowFlags::NO_NAV,
                )
                .build(|| {
                    ui.set_cursor_pos([0.0, 0.0]);
                    let draw_list = ui.get_window_draw_list();
                    let center =
                        Pos2::new(offset_x + window_size / 2.0, offset_y + window_size / 2.0);

                    if let Some(map) = self.map.as_ref() {
                        let map_image = self.textures.map.id.unwrap();
                        let map_uv = self.get_map_uv(Pos2::new(self.game.x, self.game.y));
                        draw_list
                            .add_image_rounded(
                                map_image,
                                [offset_x, offset_y],
                                [offset_x + window_size, offset_y + window_size],
                                5.0,
                            )
                            .uv_min(map_uv[0])
                            .uv_max(map_uv[1])
                            .build();

                        let teleport = self.textures.teleport.id.unwrap();
                        map.points.iter().for_each(|point| {
                            if point.icon == 0 {
                                let [min, max] = self.get_icon_pos(
                                    Pos2::new(point.x, point.y),
                                    Pos2::new(self.game.x, self.game.y),
                                    center,
                                    window_size,
                                );
                                draw_list
                                    .add_image(teleport, [min.x, min.y], [max.x, max.y])
                                    .build();
                            }
                        });

                        let [p0, p1, p2, p3] = self.arrow_to_p4(center, 32.0, self.game.angle);
                        draw_list
                            .add_image_quad(self.textures.mapplayer.id.unwrap(), p0, p1, p2, p3)
                            .build();
                    } else {
                        debug!("draw_nomap");
                    }
                });

            let logo_width = window_size * 0.5;
            let logo_height = logo_width * 0.28;

            ui.window("logo")
                .position(
                    [offset_x, offset_y + window_size - logo_height / 2.0],
                    imgui::Condition::Always,
                )
                .size([window_size, logo_height], imgui::Condition::Always)
                .flags(
                    WindowFlags::NO_TITLE_BAR
                        | WindowFlags::NO_RESIZE
                        | WindowFlags::NO_MOVE
                        | WindowFlags::NO_SCROLLBAR
                        | WindowFlags::NO_BACKGROUND
                        | WindowFlags::NO_INPUTS
                        | WindowFlags::NO_NAV,
                )
                .build(|| {
                    let draw_list = ui.get_window_draw_list();
                    let logo_offset_x = offset_x + (window_size - logo_width) / 2.0;
                    let logo_offset_y = offset_y + window_size - logo_height / 2.0;

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

impl ImguiRenderLoop for MiniMap {
    fn initialize<'a>(&'a mut self, ctx: &mut Context, render_context: &'a mut dyn RenderContext) {
        let style = ctx.style_mut();
        // let [screen_width, screen_height] = ctx.io().display_size;
        // let window_size = (screen_width * self.size).min(screen_height * self.size);
        style.window_rounding = 30.0;
        style.window_padding = [0.0, 0.0];
        self.textures.map.id = Some(
            render_context
                .load_texture(
                    self.textures.map.image.as_bytes(),
                    self.textures.map.image.width(),
                    self.textures.map.image.height(),
                )
                .unwrap(),
        );
        self.textures.teleport.id = Some(
            render_context
                .load_texture(
                    self.textures.teleport.image.as_bytes(),
                    self.textures.teleport.image.width(),
                    self.textures.teleport.image.height(),
                )
                .unwrap(),
        );
        self.textures.mapplayer.id = Some(
            render_context
                .load_texture(
                    self.textures.mapplayer.image.as_bytes(),
                    self.textures.mapplayer.image.width(),
                    self.textures.mapplayer.image.height(),
                )
                .unwrap(),
        );
        self.textures.fan.id = Some(
            render_context
                .load_texture(
                    self.textures.fan.image.as_bytes(),
                    self.textures.fan.image.width(),
                    self.textures.fan.image.height(),
                )
                .unwrap(),
        );
        self.textures.logo.id = Some(
            render_context
                .load_texture(
                    self.textures.logo.image.as_bytes(),
                    self.textures.logo.image.width(),
                    self.textures.logo.image.height(),
                )
                .unwrap(),
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
    }
}
