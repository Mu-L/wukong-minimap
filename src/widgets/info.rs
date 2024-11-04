use std::sync::Arc;

use hudhook::{
    imgui::{self, WindowFlags},
    RenderContext,
};

use crate::{widgets::Widget, wukong::Wukong};

pub struct Info;

impl Info {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for Info {
    fn render(&mut self, ui: &imgui::Ui, wukong: &Wukong) {
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
                ui.set_window_font_scale(1.5);
                ui.set_cursor_pos([20.0, 0.0]);
                ui.text_colored(
                    imgui::ImColor32::WHITE.to_rgba_f32s(),
                    // 展示地图信息保留两位小数，作者信息
                    format!(
                        "MapID:{}  X: {:.2}, Y: {:.2}, Z: {:.2} | Press M to open map | Author: jaskang | v0.1.0",
                        wukong.scene, wukong.position.x, wukong.position.y, wukong.position.z
                    ),
                );
            });
    }
}
