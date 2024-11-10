use hudhook::imgui::{
    internal::RawCast,
    sys::{ImFontAtlas_AddFontFromFileTTF, ImFontAtlas_GetGlyphRangesChineseFull},
};

use crate::widgets::{info, map};
use crate::wukong::Wukong;
use crate::{game::Game, widgets::Widget};
use hudhook::*;

pub static mut game: std::sync::LazyLock<std::sync::Mutex<Game>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Game::new()));

pub struct MapHud {
    widgets: Vec<Box<dyn Widget>>,
    pause: bool,
}

impl MapHud {
    pub fn new() -> Self {
        let widgets: Vec<Box<dyn Widget>> = vec![
            Box::new(map::MapHud::new(&wukong.areas)),
            Box::new(info::Info::new()),
        ];
        Self {
            wukong,
            widgets,
            pause: false,
        }
    }
}

impl ImguiRenderLoop for MapHud {
    fn initialize<'a>(
        &'a mut self,
        ctx: &mut imgui::Context,
        render_context: &'a mut dyn RenderContext,
    ) {
        unsafe {
            ImFontAtlas_AddFontFromFileTTF(
                ctx.fonts().raw_mut(),
                "C:\\windows\\fonts\\msyh.ttc\0".as_ptr().cast(),
                18.0,
                std::ptr::null(),
                ImFontAtlas_GetGlyphRangesChineseFull(ctx.fonts().raw_mut()),
            )
        };
        self.widgets.iter_mut().for_each(|widget| {
            widget.initialize(ctx, render_context);
        });
        let style = ctx.style_mut();
        style.window_padding = [0.00, 0.00];
    }
    fn before_render<'a>(
        &'a mut self,
        ctx: &mut imgui::Context,
        render_context: &'a mut dyn RenderContext,
    ) {
        let map_key = unsafe { game.lock().unwrap().map_id };
        for widget in self.widgets.iter_mut() {
            widget.before_render(ctx, render_context, map_key);
        }
    }
    fn render(&mut self, ui: &mut imgui::Ui) {
        self.widgets.iter_mut().for_each(|widget| {
            widget.render(ui, &self.wukong);
        });
    }
    fn message_filter(&self, _io: &imgui::Io) -> MessageFilter {
        if _io.mouse_draw_cursor {
            MessageFilter::InputAll
        } else {
            MessageFilter::empty()
        }
    }
}
