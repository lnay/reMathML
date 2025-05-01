#[allow(unused)]
#[allow(dead_code)]
use crate::mml_types::Element;
use crate::mml_types::Mi;
use tiny_skia::Pixmap;

trait Render {
    fn render(&self, scale: f64, baseline: u64) -> Pixmap;
}

impl Render for Mi {
    fn render(&self, scale: f64, baseline: u64) -> Pixmap {
        // Implementation for rendering Mi element
        unimplemented!()
    }
}
