use crate::tools::load_image;
use hudhook::{imgui, RenderContext};
use image::RgbaImage;

// pub(crate) mod info
pub(crate) mod map;

pub trait Widget: Send + Sync {
    fn initialize(&mut self, _ctx: &mut imgui::Context, _render_ctx: &mut dyn RenderContext) {}
    fn before_render(&mut self, _ctx: &mut imgui::Context, _render_ctx: &mut dyn RenderContext) {}
    fn render(&mut self, _ui: &imgui::Ui);
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
