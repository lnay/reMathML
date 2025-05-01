#[allow(unused)]
#[allow(dead_code)]
use crate::mml_types::Element;
use crate::mml_types::{Mi, Mn, Mo, Msub, Msup};
use crate::text_rendering::render_text;
use tiny_skia::{ALPHA_TRANSPARENT, Pixmap, PixmapPaint, PixmapRef, Transform};

pub trait Render {
    fn render(&self, font_size: f32, baseline: u64) -> Pixmap;
}

impl Render for Mi {
    fn render(&self, font_size: f32, _baseline: u64) -> Pixmap {
        render_text(self.identifier.clone(), font_size)
    }
}

impl Render for Mn {
    fn render(&self, font_size: f32, _baseline: u64) -> Pixmap {
        render_text(self.number.clone(), font_size)
    }
}
impl Render for Mo {
    fn render(&self, font_size: f32, _baseline: u64) -> Pixmap {
        render_text(self.operator.clone(), font_size)
    }
}
impl Render for Msup {
    fn render(&self, font_size: f32, _baseline: u64) -> Pixmap {
        let paint = PixmapPaint::default();
        let transform = Transform::default();
        const SUPERSCRIPT_FONT_RATIO: f32 = 0.7;
        const SUPERSCRIPT_VERTICAL_OFFSET: f32 = 0.5;
        let base = self.base.render(font_size, _baseline);
        let superscript = self
            .superscript
            .render(font_size * SUPERSCRIPT_FONT_RATIO, _baseline);
        let width = base.width() + superscript.width();
        let height = base.height().max(superscript.height() * 2);
        let base_y_offset = 0.max(superscript.height() - base.height() / 2) as i32;
        let mut pixmap = Pixmap::new(width, height).unwrap();
        let base_width = base.width() as i32;
        pixmap.draw_pixmap(0, base_y_offset, base.as_ref(), &paint, transform, None);
        pixmap.draw_pixmap(base_width, 0, superscript.as_ref(), &paint, transform, None);
        pixmap
    }
}
impl Render for Msub {
    fn render(&self, font_size: f32, _baseline: u64) -> Pixmap {
        let paint = PixmapPaint::default();
        let transform = Transform::default();
        const SUBSCRIPT_FONT_RATIO: f32 = 0.7;
        const SUPERSCRIPT_VERTICAL_OFFSET: f32 = 0.5;
        let base = self.base.render(font_size, _baseline);
        let subscript = self
            .subscript
            .render(font_size * SUBSCRIPT_FONT_RATIO, _baseline);
        let width = base.width() + subscript.width();
        let height = base.height().max(2 * subscript.height());
        let mut pixmap = Pixmap::new(width, height).unwrap();

        let base_y_offset = 0.max(subscript.height() - base.height() / 2) as i32;
        let subscript_y_offset = base_y_offset + (base.height() / 2) as i32;
        let base_width = base.width() as i32;
        pixmap.draw_pixmap(0, base_y_offset, base.as_ref(), &paint, transform, None);
        pixmap.draw_pixmap(
            base_width,
            subscript_y_offset,
            subscript.as_ref(),
            &paint,
            transform,
            None,
        );
        pixmap
    }
}
impl Render for Element {
    fn render(&self, font_size: f32, _baseline: u64) -> Pixmap {
        match self {
            Element::Mi(mi) => mi.render(font_size, _baseline),
            Element::Mn(mn) => mn.render(font_size, _baseline),
            Element::Mo(mo) => mo.render(font_size, _baseline),
            Element::Msup(msup) => msup.render(font_size, _baseline),
            Element::Msub(msub) => msub.render(font_size, _baseline),
            _ => unimplemented!("Render not implemented for all MathML node types"),
        }
    }
}
