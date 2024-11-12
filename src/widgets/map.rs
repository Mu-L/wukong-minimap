use std::cell::OnceCell;
use std::collections::HashMap;

use hudhook::RenderContext;
use hudhook::{
    imgui::{self, Condition, Image, Key, TextureId, WindowFlags},
    tracing::debug,
};
use image::{EncodableLayout, RgbaImage};
use serde::{Deserialize, Serialize};

use crate::game::Game;
use crate::tools::{load_image, load_json, point_inside};
use crate::widgets::{ImageTexture, Widget};

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

#[derive(Debug, Clone, Copy)]
struct IconCheckbox {
    key: &'static str,
    name: &'static str,
    checked: bool,
}
impl IconCheckbox {
    fn new(key: &'static str, name: &'static str, checked: bool) -> Self {
        IconCheckbox { key, name, checked }
    }
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

pub struct MapHud {
    textures: HashMap<String, ImageTexture>,
    icons: Vec<IconCheckbox>,
    areas: Vec<AreaInfo>,
    area: Option<AreaInfo>,
    pre_area_id: i32,
}

impl MapHud {
    pub fn new() -> Self {
        let areas: Vec<AreaInfo> = load_json("areas.json");
        let icons: Vec<IconCheckbox> = vec![
            IconCheckbox::new("teleport", "传送点", true),
            IconCheckbox::new("start", "入口", true),
            IconCheckbox::new("end", "出口", true),
            IconCheckbox::new("pass-route", "捷径", true),
            IconCheckbox::new("hidden", "隐藏点", true),
            IconCheckbox::new("boss", "妖王", true),
            IconCheckbox::new("toumu", "头目", true),
            IconCheckbox::new("renwu", "人物", true),
            IconCheckbox::new("zhixian", "支线", true),
            IconCheckbox::new("dazuo", "打坐", true),
            IconCheckbox::new("baoxiang", "宝箱", true),
            IconCheckbox::new("bianhua", "变化", true),
            IconCheckbox::new("fabao", "法宝", true),
            IconCheckbox::new("pigua", "披挂", true),
            IconCheckbox::new("zhenwan", "珍玩", true),
            IconCheckbox::new("hulu", "葫芦", true),
            IconCheckbox::new("jiushi", "酒食", true),
            IconCheckbox::new("xiandan", "仙丹", false),
            IconCheckbox::new("yongpin", "用品", false),
            IconCheckbox::new("yaopin", "药品", false),
            IconCheckbox::new("cailiao", "材料", false),
            IconCheckbox::new("jingpo", "精魄", false),
            IconCheckbox::new("yaocai", "药材", false),
            IconCheckbox::new("luojia", "落伽香藤", false),
            IconCheckbox::new("sandongchong", "三冬虫", false),
            IconCheckbox::new("lingyun", "灵蕴", false),
        ];

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

        Self {
            textures,
            icons,
            areas,
            area: None,
            pre_area_id: 0,
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
    fn render_map(&mut self, ui: &imgui::Ui) {
        let game = Game::get_game();
        if game.enable && !game.paused {
            let display_size = ui.io().display_size;
            let window_size = display_size[1] * MINI_MAP_SIZE;
            let [position_x, position_y] = [display_size[0] - window_size - 10.0, 10.0];
            ui.window("mini_map")
                .size([window_size, window_size], Condition::Always)
                .position([position_x, position_y], Condition::Always)
                .flags(
                    WindowFlags::NO_TITLE_BAR
                        | WindowFlags::NO_RESIZE
                        | WindowFlags::NO_MOVE
                        | WindowFlags::NO_SCROLLBAR,
                )
                .bg_alpha(0.8)
                .build(|| {
                    let txt_id = self.get_texture_id("map").unwrap();
                    let area = self.get_area(game.level, &[game.x, game.y, game.z]);

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

        if game.enable && !game.paused && game.open {
            let display_size = ui.io().display_size;
            let window_size = f32::min(display_size[0], display_size[1]) * MAIN_MAP_SIZE;

            let [position_x, position_y] = [
                (display_size[0] - window_size) / 2.0,
                (display_size[1] - window_size) / 2.0,
            ];
            ui.window("main_map")
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

            ui.window("main_control")
                .size([100.0, window_size], Condition::Always) // 修改位置到右上角
                .position([position_x - 100.0, position_y], Condition::Always)
                .flags(
                    WindowFlags::NO_TITLE_BAR
                        | WindowFlags::NO_RESIZE
                        | WindowFlags::NO_MOVE
                        | WindowFlags::NO_SCROLLBAR,
                )
                .build(|| {
                    // 每个 icon 都是一个 checkbox
                    self.icons.iter_mut().for_each(|icon| {
                        ui.checkbox(icon.name, &mut icon.checked);
                    });
                });
        }
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

impl Widget for MapHud {
    fn initialize(&mut self, _ctx: &mut imgui::Context, render_ctx: &mut dyn RenderContext) {
        self.textures.iter_mut().for_each(|(_, txt)| {
            txt.id = render_ctx
                .load_texture(txt.image.as_bytes(), txt.image.width(), txt.image.height())
                .ok();
        });
    }
    fn before_render(&mut self, ctx: &mut imgui::Context, render_ctx: &mut dyn RenderContext) {
        // ctx.io_mut().mouse_draw_cursor = self.open;
        let game = Game::get_game();
        self.area = self.get_area(game.level, &[game.x, game.y, game.z]);
        let area_id = self.area.as_ref().map(|a| a.id).unwrap_or(0);

        if area_id != self.pre_area_id {
            self.pre_area_id = area_id;
            let map_texture = self.textures.get("map").unwrap();
            let data = match self.get_map_image(area_id) {
                Some(data) => data.as_bytes(),
                None => map_texture.image.as_bytes(),
            };
            println!("replace_texture: {:?}", area_id);
            let _ = render_ctx.replace_texture(map_texture.id.unwrap(), data, 2000, 2000);
        }
    }
    fn render(&mut self, ui: &imgui::Ui) {
        self.render_map(ui);
    }
}
