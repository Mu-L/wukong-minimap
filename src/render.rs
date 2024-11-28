use std::collections::HashMap;
use std::{cell::OnceCell, sync::Mutex};

use gilrs::{Button, EventType, Gilrs};

use hudhook::{
    imgui::{
        self,
        internal::RawCast,
        sys::{ImFontAtlas_AddFontFromFileTTF, ImFontAtlas_GetGlyphRangesChineseFull},
        Condition, Context, Image, Io, TextureId, WindowFlags,
    },
    tracing::debug,
    ImguiRenderLoop, MessageFilter, RenderContext,
};
use image::{EncodableLayout, RgbaImage};
use serde::{Deserialize, Serialize};

use crate::{
    data::get_icons,
    tools::{load_image, load_json, point_inside},
    wukong::GameState,
};
use crate::{data::IconCheckbox, wukong::Wukong};

static mut MAP_IMAGES: OnceCell<HashMap<i32, RgbaImage>> = OnceCell::new();

// 小地图视窗大小, 对应游戏坐标
const MAP_VIEWPORT: f32 = 20000.0;
// 小地图窗口大小
const MINI_MAP_SIZE: f32 = 0.2;
// 大地图窗口大小
const MAIN_MAP_SIZE: f32 = 0.8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Icon {
    pub name: String,
    pub category: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaInfo {
    pub id: i32,
    pub map: i32,
    pub code: String,
    pub name: String,
    pub image: String,
    pub range_x: [f32; 2],
    pub range_y: [f32; 2],
    pub range_z: [f32; 2],
    pub range_side: Option<[f32; 4]>,
    pub points: Vec<Icon>,
}

impl AreaInfo {
    pub fn width(&self) -> f32 {
        self.range_x[1] - self.range_x[0]
    }
    pub fn height(&self) -> f32 {
        self.range_y[1] - self.range_y[0]
    }
}

fn load_map_data(areas: &Vec<AreaInfo>) -> HashMap<i32, RgbaImage> {
    let mut maps: HashMap<i32, RgbaImage> = HashMap::new();
    for area in areas {
        let image_data = load_image(&area.image);
        maps.insert(area.id, image_data);
    }
    maps
}

pub struct ImageTexture {
    pub id: Option<imgui::TextureId>,
    pub image: RgbaImage,
}

impl ImageTexture {
    pub fn new(str: &str) -> Self {
        Self {
            id: None,
            image: load_image(str),
        }
    }
}

pub struct MapHud {
    gilrs: Mutex<Gilrs>,
    textures: HashMap<String, ImageTexture>,
    icons: Vec<IconCheckbox>,
    areas: Vec<AreaInfo>,
    area: Option<AreaInfo>,
    pre_area_id: i32,
    open_mainmap: bool,
    enable: bool,
}

impl MapHud {
    pub fn new() -> Self {
        let areas: Vec<AreaInfo> = load_json("areas.json");
        let icons = get_icons();

        let mut textures = HashMap::new();
        textures.insert("map".to_string(), ImageTexture::new("nomap.png"));
        textures.insert("arrow".to_string(), ImageTexture::new("arrow.png"));

        icons.iter().for_each(|icon| {
            textures.insert(
                icon.key.to_string(),
                ImageTexture::new(format!("icon_{}.png", icon.key).as_str()),
            );
        });

        let maps = load_map_data(&areas);
        unsafe { MAP_IMAGES.get_or_init(|| maps) };

        let gilrs = Gilrs::new().unwrap();
        Self {
            gilrs: Mutex::new(gilrs),
            textures,
            icons,
            areas,
            area: None,
            pre_area_id: 0,
            open_mainmap: false,
            enable: true,
        }
    }
    fn get_texture_id(&self, name: &str) -> Option<TextureId> {
        self.textures.get(name).map(|t| t.id.unwrap())
    }
    fn get_map_image(&self, level: i32) -> Option<&RgbaImage> {
        if level == 0 {
            return None;
        }
        unsafe { MAP_IMAGES.get().unwrap().get(&level) }
    }
    fn is_icon_checked(&self, key: &str) -> bool {
        self.icons
            .iter()
            .any(|icon| icon.checked && icon.key == key)
    }
    fn get_area(&self, map_id: i32, position: &[f32; 3]) -> Option<AreaInfo> {
        let area = self
            .areas
            .iter()
            .rfind(|m| {
                if map_id >= 10
                    && m.map == map_id
                    && position[0] >= m.range_x[0]
                    && position[0] <= m.range_x[1]
                    && position[1] >= m.range_y[0]
                    && position[1] <= m.range_y[1]
                    && position[0] != 0.0
                    && position[1] != 0.0
                {
                    if let Some(range_side) = &m.range_side {
                        let inside = point_inside(
                            [range_side[0], range_side[1]],
                            [range_side[2], range_side[3]],
                            [position[0], position[1]],
                        );
                        debug!("inside: {:?}", inside);
                        if inside {
                            return true;
                        }
                    }
                    if position[2] >= m.range_z[0] && position[2] <= m.range_z[1] {
                        return true;
                    }
                }
                false
            })
            .map(|m| m.clone());
        area
    }

    // 转换游戏坐标转为坐标百分比
    pub fn point_to_percent(&self, point: [f32; 2], area: &AreaInfo) -> [f32; 2] {
        let x = (point[0] - area.range_x[0]) / area.width();
        let y = (point[1] - area.range_y[0]) / area.height();
        [x, y]
    }
    fn render_minimap(&mut self, ui: &imgui::Ui, game: &GameState) {
        if game.map_id > 1 && game.playing {
            let display_size = ui.io().display_size;
            let window_size = display_size[1] * MINI_MAP_SIZE;
            let [position_x, position_y] = [display_size[0] - window_size - 10.0, 10.0];
            ui.window("minimap")
                .size([window_size, window_size], Condition::Always)
                .position([position_x, position_y], Condition::Always)
                .flags(
                    WindowFlags::NO_TITLE_BAR
                        | WindowFlags::NO_RESIZE
                        | WindowFlags::NO_MOVE
                        | WindowFlags::NO_SCROLLBAR,
                )
                .bg_alpha(0.6)
                .build(|| {
                    let txt_id = self.get_texture_id("map").unwrap();
                    let area = self.get_area(game.map_id, &[game.x, game.y, game.z]);

                    if let Some(area) = area {
                        // 将游戏坐标转换为小地图坐标
                        let minimap_x = (game.x - area.range_x[0]) / area.width();
                        let minimap_y = (game.y - area.range_y[0]) / area.height();

                        debug!("draw_minimap");
                        let [uv0, uv1] = self.point_to_minimap_uv(
                            [minimap_x, minimap_y],
                            [area.width(), area.height()],
                            window_size,
                        );

                        Image::new(txt_id, [window_size, window_size])
                            .uv0(uv0)
                            .uv1(uv1)
                            .build(ui);

                        area.points
                            .iter()
                            .filter(|p| self.is_icon_checked(p.category.as_str()))
                            .for_each(|p| {
                                if let Some(tid) = self.get_texture_id(p.category.as_str()) {
                                    // 计算位置后 画传送点图标。
                                    let scale = window_size / MAP_VIEWPORT;
                                    let [x, y] = self.point_to_percent([p.x, p.y], &area);
                                    let [minimap_width, minimap_height] =
                                        [area.width() * scale, area.height() * scale];
                                    let [x, y] = [
                                        (x - uv0[0]) * minimap_width,
                                        (y - uv0[1]) * minimap_height,
                                    ];

                                    ui.set_cursor_pos([x - 18.0, y - 32.0]);
                                    Image::new(tid, [36.0, 48.0]).build(ui);
                                }
                            });

                        let draw_list = ui.get_window_draw_list();

                        // let [win_x, win_y] = ui.window_pos();
                        let center = [
                            position_x + window_size / 2.0,
                            position_y + window_size / 2.0,
                        ];
                        let [p0, p1, p2, p3] =
                            self.arrow_to_p4(ui, center, 32.0, game.angle + 180.0);

                        // 使用 add_image_quad 方法角色箭头图标
                        draw_list
                            .add_image_quad(self.get_texture_id("arrow").unwrap(), p0, p1, p2, p3)
                            .build();
                    } else {
                        debug!("draw_nomap");
                        Image::new(txt_id, [window_size, window_size])
                            .uv0([0.0, 0.0])
                            .uv1([1.0, 1.0])
                            .build(ui);
                    }
                });
        }
    }
    fn render_mainmap(&mut self, ui: &imgui::Ui, game: &GameState) {
        if game.map_id > 1 && game.playing {
            let display_size = ui.io().display_size;
            let window_size = f32::min(display_size[0], display_size[1]) * MAIN_MAP_SIZE;

            let [position_x, position_y] = [
                (display_size[0] - window_size) / 2.0,
                (display_size[1] - window_size) / 2.0,
            ];
            ui.window("mainmap")
                .size([window_size, window_size], Condition::Always)
                .position([position_x, position_y], Condition::Always)
                .flags(WindowFlags::NO_TITLE_BAR | WindowFlags::NO_RESIZE | WindowFlags::NO_MOVE)
                .bg_alpha(0.8)
                .build(|| {
                    let txt_id = self.get_texture_id("map").unwrap();
                    ui.set_cursor_pos([0.0, 0.0]);

                    if let Some(area) = &self.area {
                        debug!("draw_mainmap");
                        Image::new(txt_id, [window_size, window_size]).build(ui);

                        let draw_list = ui.get_window_draw_list();

                        area.points
                            .iter()
                            .filter(|p| self.is_icon_checked(p.category.as_str()))
                            .for_each(|p| {
                                if let Some(tid) = self.get_texture_id(p.category.as_str()) {
                                    // 计算位置后 画传送点图标。
                                    let [x, y] = self.point_to_percent([p.x, p.y], &area);
                                    let [x, y] = [x * window_size, y * window_size];
                                    ui.set_cursor_pos([x - 18.0, y - 32.0]);
                                    Image::new(tid, [36.0, 48.0]).build(ui);
                                    // self.render_centered_text(ui, &p.name, x, y + 24.0);
                                }
                            });

                        let location = self.point_to_percent([game.x, game.y], &area);
                        let location = [
                            position_x + location[0] * window_size,
                            position_y + location[1] * window_size,
                        ];
                        let [p0, p1, p2, p3] =
                            self.arrow_to_p4(ui, location, 32.0, game.angle + 180.0);

                        // 使用 add_image_quad 方法角色箭头图标
                        draw_list
                            .add_image_quad(self.get_texture_id("arrow").unwrap(), p0, p1, p2, p3)
                            .build();
                    } else {
                        debug!("draw_nomap");
                        Image::new(txt_id, [window_size, window_size])
                            .uv0([0.0, 0.0])
                            .uv1([1.0, 1.0])
                            .build(ui);
                    }
                });
            ui.window("position")
                .size([window_size, 30.0], Condition::Always) // 修改位置到右上角
                .position([position_x, position_y + window_size], Condition::Always)
                .flags(
                    WindowFlags::NO_TITLE_BAR
                        | WindowFlags::NO_RESIZE
                        | WindowFlags::NO_MOVE
                        | WindowFlags::NO_SCROLLBAR,
                )
                .bg_alpha(0.6)
                .build(|| {
                    ui.set_window_font_scale(1.0);
                    let text = format!(
                        "map:{} id:{} x:{:.2} y:{:.2} z:{:.2} by jaskang",
                        game.level_name, game.map_id, game.x, game.y, game.z
                    );
                    // 计算文本尺寸
                    let text_size = ui.calc_text_size(&text);
                    ui.set_cursor_pos([10.0, (30.0 - text_size[1]) * 0.5]);
                    ui.text_colored(
                        imgui::ImColor32::WHITE.to_rgba_f32s(),
                        // 展示地图信息保两位小数
                        text,
                    );
                });

            // ui.window("main_control")
            //     .size([100.0, window_size], Condition::Always) // 修改位置到右上角
            //     .position([position_x - 100.0, position_y], Condition::Always)
            //     .flags(
            //         WindowFlags::NO_TITLE_BAR
            //             | WindowFlags::NO_RESIZE
            //             | WindowFlags::NO_MOVE
            //             | WindowFlags::NO_SCROLLBAR,
            //     )
            //     .build(|| {
            //         // 每个 icon 都是一个 checkbox
            //         self.icons.iter_mut().for_each(|icon| {
            //             ui.checkbox(icon.name, &mut icon.checked);
            //         });
            //     });
        }
    }
    fn render_info(&mut self, ui: &imgui::Ui, game: &GameState) {
        let display_size = ui.io().display_size;
        let [position_x, position_y] = [0.0, display_size[1] - 30.0];

        ui.window("info")
            .position([position_x, position_y], imgui::Condition::FirstUseEver)
            .size([display_size[0], 30.0], imgui::Condition::FirstUseEver)
            .flags(
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_SCROLLBAR
                    | WindowFlags::NO_BACKGROUND,
            )
            .build(|| {
                ui.set_window_font_scale(1.0);
                let text = format!(
                    "M: Open | N: Disable | black-myth-map v0.4.0 | {}",
                    game.level_name
                );
                let text_size = ui.calc_text_size(&text);
                ui.set_cursor_pos([10.0, (30.0 - text_size[1]) * 0.5]);
                ui.text_colored(imgui::ImColor32::WHITE.to_rgba_f32s(), text);
            });
    }
    /**
     * 获取地图背景图片的偏移值, 人物总是展示在中心位置,不断的调整背景图片的坐标值实现实时小地图功能,
     * 地图视窗大小为 MAP_WINDOW_SIZE, 展示游戏坐标范围为 MAP_VIEWPORT.
     * 返回地图图片的 uv 坐标
     */
    fn point_to_minimap_uv(
        &self,
        point: [f32; 2],
        size: [f32; 2],
        window_size: f32,
    ) -> [[f32; 2]; 2] {
        let half_window_size = window_size / 2.0;
        let scale = window_size / MAP_VIEWPORT;
        let (minimap_width, minimap_height) = (size[0] * scale, size[1] * scale);
        let start_x = (point[0] * minimap_width - half_window_size) / minimap_width;
        let end_x = (point[0] * minimap_width + half_window_size) / minimap_width;
        let start_y = (point[1] * minimap_height - half_window_size) / minimap_height;
        let end_y = (point[1] * minimap_height + half_window_size) / minimap_height;
        [[start_x, start_y], [end_x, end_y]]
    }

    fn arrow_to_p4(
        &self,
        ui: &imgui::Ui,
        location: [f32; 2],
        size: f32,
        angle: f32,
    ) -> [[f32; 2]; 4] {
        ui.set_cursor_pos(location);
        let half_size = size / 2.0;

        // 计算旋转后的四个角点
        let cos_angle = angle.to_radians().cos();
        let sin_angle = angle.to_radians().sin();

        // 计算四个角点的位置
        let p1 = [
            location[0] + (-half_size * cos_angle - -half_size * sin_angle),
            location[1] + (-half_size * sin_angle + -half_size * cos_angle),
        ];
        let p2 = [
            location[0] + (half_size * cos_angle - -half_size * sin_angle),
            location[1] + (half_size * sin_angle + -half_size * cos_angle),
        ];
        let p3 = [
            location[0] + (half_size * cos_angle - half_size * sin_angle),
            location[1] + (half_size * sin_angle + half_size * cos_angle),
        ];
        let p4 = [
            location[0] + (-half_size * cos_angle - half_size * sin_angle),
            location[1] + (-half_size * sin_angle + half_size * cos_angle),
        ];
        [p1, p2, p3, p4]
    }
}

impl ImguiRenderLoop for MapHud {
    fn initialize<'a>(&'a mut self, ctx: &mut Context, render_context: &'a mut dyn RenderContext) {
        unsafe {
            ImFontAtlas_AddFontFromFileTTF(
                ctx.fonts().raw_mut(),
                "C:\\windows\\fonts\\msyh.ttc\0".as_ptr().cast(),
                18.0,
                std::ptr::null(),
                ImFontAtlas_GetGlyphRangesChineseFull(ctx.fonts().raw_mut()),
            )
        };
        let style = ctx.style_mut();
        style.window_padding = [0.00, 0.00];

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
        let game = Wukong::game_state();
        self.area = self.get_area(game.map_id, &[game.x, game.y, game.z]);
        let area_id = self.area.as_ref().map(|a| a.id).unwrap_or(0);

        if area_id != self.pre_area_id {
            self.pre_area_id = area_id;
            let map_texture = self.textures.get("map").unwrap();
            let data = match self.get_map_image(area_id) {
                Some(data) => data.as_bytes(),
                None => map_texture.image.as_bytes(),
            };
            println!("replace_texture: {:?}", area_id);
            let _ = render_context.replace_texture(map_texture.id.unwrap(), data, 2000, 2000);
        }
    }
    fn render(&mut self, ui: &mut imgui::Ui) {
        let game = Wukong::game_state();
        // Examine new events
        if let Ok(mut gilrs) = self.gilrs.lock() {
            while let Some(event) = gilrs.next_event() {
                match event.event {
                    EventType::ButtonPressed(Button::Select, _) => {
                        if !self.enable {
                            self.enable = true;
                        }
                        self.open_mainmap = !self.open_mainmap;
                    }
                    _ => {}
                }
            }
        }

        if ui.is_key_pressed_no_repeat(imgui::Key::N) {
            self.enable = !self.enable;
        }
        if ui.is_key_pressed_no_repeat(imgui::Key::M) {
            // 开启大地图时, 如果小地图未开启, 则自动开启小地图
            if !self.enable {
                self.enable = true;
            }
            self.open_mainmap = !self.open_mainmap;
        }
        if self.enable {
            self.render_info(ui, &game);
            self.render_minimap(ui, &game);
            if self.open_mainmap {
                self.render_mainmap(ui, &game);
            }
        }
    }
    fn message_filter(&self, _io: &Io) -> MessageFilter {
        if _io.mouse_draw_cursor {
            MessageFilter::InputAll
        } else {
            MessageFilter::empty()
        }
    }
}
