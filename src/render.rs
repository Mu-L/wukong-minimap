use std::cell::OnceCell;
use std::collections::HashMap;

use crate::{
    utils::{image_with_bytes, image_with_file, load_data, MapInfo, Pos2},
    wukong::{self, GameState},
};
use hudhook::{
    imgui::{self, Condition, Context, Image, StyleVar, TextureId, WindowFlags},
    tracing::debug,
    ImguiRenderLoop, RenderContext,
};
use image::{EncodableLayout, RgbaImage};
use serde::{Deserialize, Serialize};

static mut MAP_IMAGES: OnceCell<HashMap<i32, RgbaImage>> = OnceCell::new();

// 小地图视窗大小, 对应游戏坐标
const MAP_VIEWPORT: f32 = 20000.0;
// 小地图窗口大小
const MINI_MAP_SIZE: f32 = 0.2;

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

pub struct MiniMap {
    textures: HashMap<String, ImageTexture>,
    zoom: f32,
    map: Option<MapInfo>,
    maps: Vec<MapInfo>,
}

impl MiniMap {
    pub fn new() -> Self {
        wukong::init();

        let maps: Vec<MapInfo> = load_data();

        let mut textures = HashMap::new();
        textures.insert(
            "bilibili".to_string(),
            ImageTexture::with_bytes(include_bytes!("../includes/bilibili.png")),
        );
        textures.insert(
            "mapplayer".to_string(),
            ImageTexture::with_bytes(include_bytes!("../includes/mapplayer.png")),
        );
        textures.insert(
            "fan".to_string(),
            ImageTexture::with_bytes(include_bytes!("../includes/fan.png")),
        );
        textures.insert(
            "teleport".to_string(),
            ImageTexture::with_bytes(include_bytes!("../includes/teleport.png")),
        );

        // 加载地图图片
        maps.iter().for_each(|map| {
            textures.insert(map.key.clone(), ImageTexture::with_file(map.key.as_str()));
        });

        Self {
            textures,
            zoom: 0.1,
            map: None,
            maps,
        }
    }

    fn update_map(&mut self, game_state: &GameState) -> bool {
        let level = game_state.level.clone();
        let map = self
            .maps
            .iter()
            .rev() // 从后面开始查找
            .find(|map| {
                if map.level == level
                    && game_state.x >= map.range.start[0]
                    && game_state.x <= map.range.end[0]
                    && game_state.y >= map.range.start[1]
                    && game_state.y <= map.range.end[1]
                    && game_state.z >= map.range.start[2]
                    && game_state.z <= map.range.end[2]
                {
                    return if map.areas.is_empty() {
                        true
                    } else {
                        map.areas.iter().any(|area| {
                            game_state.x >= area.start[0]
                                && game_state.x <= area.end[0]
                                && game_state.y >= area.start[1]
                                && game_state.y <= area.end[1]
                                && game_state.z >= area.start[2]
                                && game_state.z <= area.end[2]
                        })
                    };
                }
                false
            })
            .cloned();

        match (self.map.as_ref(), map) {
            (_, None) => {
                self.map = None;
                false
            }
            (None, Some(new_map)) => {
                self.map = Some(new_map);
                true
            }
            (Some(current_map), Some(new_map)) => {
                if current_map.key != new_map.key {
                    self.map = Some(new_map);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn get_texture(&self, name: &str) -> Option<TextureId> {
        self.textures.get(name).map(|t| t.id.unwrap())
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
        let half_size = size / 2.0;

        // 计算旋转后的四个角点
        let cos_angle = angle.to_radians().cos();
        let sin_angle = angle.to_radians().sin();

        // 计算四个角点的位置
        let p1 = [
            location.x + (-half_size * cos_angle - -half_size * sin_angle),
            location.y + (-half_size * sin_angle + -half_size * cos_angle),
        ];
        let p2 = [
            location.x + (half_size * cos_angle - -half_size * sin_angle),
            location.y + (half_size * sin_angle + -half_size * cos_angle),
        ];
        let p3 = [
            location.x + (half_size * cos_angle - half_size * sin_angle),
            location.y + (half_size * sin_angle + half_size * cos_angle),
        ];
        let p4 = [
            location.x + (-half_size * cos_angle - half_size * sin_angle),
            location.y + (-half_size * sin_angle + half_size * cos_angle),
        ];
        [p1, p2, p3, p4]
    }
    fn render(&mut self, ui: &imgui::Ui, game: &GameState) {
        if game.playing {
            let [screen_width, screen_height] = ui.io().display_size;
            let window_size = (screen_width * 0.15).min(screen_height * 0.15);

            let [offset_x, offset_y] = [screen_width - window_size - 10.0, 10.0];
            let _ = ui.push_style_var(StyleVar::WindowPadding([0.0, 0.0]));
            let _ = ui.push_style_var(StyleVar::WindowRounding(5.0));
            ui.window("wukong-minimap")
                .size([window_size, window_size], Condition::Appearing)
                .position([offset_x, offset_y], Condition::Appearing)
                .flags(
                    WindowFlags::NO_TITLE_BAR
                        | WindowFlags::NO_RESIZE
                        | WindowFlags::NO_MOVE
                        | WindowFlags::NO_SCROLLBAR
                        | WindowFlags::NO_BACKGROUND,
                )
                .build(|| {
                    ui.set_cursor_pos([0.0, 0.0]);
                    let draw_list = ui.get_window_draw_list();
                    let center =
                        Pos2::new(offset_x + window_size / 2.0, offset_y + window_size / 2.0);

                    if let Some(map) = self.map.as_ref() {
                        let map_image = self.get_texture(&map.key).unwrap();
                        let map_uv = self.get_map_uv(Pos2::new(game.x, game.y));
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

                        let teleport = self.get_texture("teleport").unwrap();
                        map.points.iter().for_each(|point| {
                            if point.icon == 0 {
                                let [min, max] = self.get_icon_pos(
                                    Pos2::new(point.x, point.y),
                                    Pos2::new(game.x, game.y),
                                    center,
                                    window_size,
                                );
                                draw_list
                                    .add_image(teleport, [min.x, min.y], [max.x, max.y])
                                    .build();
                            }
                        });

                        let mapplayer = self.get_texture("mapplayer").unwrap();
                        let [p0, p1, p2, p3] = self.arrow_to_p4(center, 32.0, game.angle);
                        draw_list.add_image_quad(mapplayer, p0, p1, p2, p3).build();
                    } else {
                        debug!("draw_nomap");
                    }
                });

            let logo_width = window_size * 0.5;
            let logo_height = logo_width * 0.28;

            ui.window("logo")
                .position(
                    [offset_x, offset_y + window_size - logo_height / 2.0],
                    imgui::Condition::Appearing,
                )
                .size([window_size, logo_height], imgui::Condition::Appearing)
                .flags(
                    WindowFlags::NO_TITLE_BAR
                        | WindowFlags::NO_RESIZE
                        | WindowFlags::NO_MOVE
                        | WindowFlags::NO_SCROLLBAR
                        | WindowFlags::NO_BACKGROUND,
                )
                .build(|| {
                    let bilibili = self.get_texture("bilibili").unwrap();
                    let draw_list = ui.get_window_draw_list();
                    let logo_offset_x = offset_x + (window_size - logo_width) / 2.0;
                    let logo_offset_y = offset_y + window_size - logo_height / 2.0;

                    draw_list
                        .add_image(
                            bilibili,
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

    // /**
    //  * 获取地图背景图片的偏移值, 人物总是展示在中心位置,不断的调整背景图片的坐标值实现实时小地图功能,
    //  * 地图视窗大小为 MAP_WINDOW_SIZE, 展示游戏坐标范围为 MAP_VIEWPORT.
    //  * 返回地图图片的 uv 坐标
    //  */
    // fn point_to_minimap_uv(
    //     &self,
    //     point: [f32; 2],
    //     size: [f32; 2],
    //     window_size: f32,
    // ) -> [[f32; 2]; 2] {
    //     let half_window_size = window_size / 2.0;
    //     let scale = window_size / MAP_VIEWPORT;
    //     let (minimap_width, minimap_height) = (size[0] * scale, size[1] * scale);
    //     let start_x = (point[0] * minimap_width - half_window_size) / minimap_width;
    //     let end_x = (point[0] * minimap_width + half_window_size) / minimap_width;
    //     let start_y = (point[1] * minimap_height - half_window_size) / minimap_height;
    //     let end_y = (point[1] * minimap_height + half_window_size) / minimap_height;
    //     [[start_x, start_y], [end_x, end_y]]
    // }
}

impl ImguiRenderLoop for MiniMap {
    fn initialize<'a>(&'a mut self, ctx: &mut Context, render_context: &'a mut dyn RenderContext) {
        self.textures.iter_mut().for_each(|(_, txt)| {
            txt.id = render_context
                .load_texture(txt.image.as_bytes(), txt.image.width(), txt.image.height())
                .ok();
        });
    }
    fn before_render<'a>(
        &'a mut self,
        _ctx: &mut Context,
        render_context: &'a mut dyn RenderContext,
    ) {
        // ctx.io_mut().mouse_draw_cursor = self.open;
        let game = wukong::game_state();
        let _ = self.update_map(&game);
    }
    fn render(&mut self, ui: &mut imgui::Ui) {
        let game = wukong::game_state();
        self.render(ui, &game);
    }
}
