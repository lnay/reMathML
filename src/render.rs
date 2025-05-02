use std::cmp::Ordering;

#[allow(unused)]
#[allow(dead_code)]
use crate::mml_types::Element;
use crate::mml_types::{Mfrac, Mi, Mn, Mo, Msub, Msup};
use crate::text_rendering::render_text;
use tiny_skia::{
    ALPHA_TRANSPARENT, IntRect, Paint, Pixmap, PixmapPaint, PixmapRef, Rect, Transform,
};

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
impl Render for Mfrac {
    fn render(&self, font_size: f32, _baseline: u64) -> Pixmap {
        let paint = PixmapPaint::default();
        let transform = Transform::default();
        let line_width = (font_size / 10.0).ceil() as u32;
        let numerator = self.numer.render(font_size, _baseline);
        let denominator = self.denom.render(font_size, _baseline);
        let width = numerator.width().max(denominator.width());
        let term_height = numerator.height().max(denominator.height()) as u32;
        let height = (2 * term_height + line_width) as u32;

        let mut pixmap = Pixmap::new(width, height).unwrap();

        let (numerator_x_offset, denominator_x_offset) =
            match numerator.width().cmp(&denominator.width()) {
                Ordering::Less => ((denominator.width() - numerator.width()) / 2, 0),
                Ordering::Equal => (0, 0),
                Ordering::Greater => (0, (numerator.width() - denominator.width()) / 2),
            };

        pixmap.draw_pixmap(
            numerator_x_offset as i32,
            (term_height - numerator.height()) as i32,
            numerator.as_ref(),
            &paint,
            transform,
            None,
        );
        pixmap.draw_pixmap(
            denominator_x_offset as i32,
            (term_height + line_width) as i32,
            denominator.as_ref(),
            &paint,
            transform,
            None,
        );
        pixmap.fill_rect(
            IntRect::from_ltrb(
                0,
                term_height as i32,
                width as i32,
                (term_height + line_width) as i32,
            )
            .unwrap()
            .to_rect(),
            &Paint::default(),
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
#[cfg(test)]
mod tests {
    use super::*;
    use function_name::named;

    #[named]
    #[test]
    fn beta_sub_alpha_sup_2() {
        let alpha = Mi {
            identifier: "α".into(),
        };
        let beta = Mi {
            identifier: "β".into(),
        };
        let number = Mn { number: "2".into() };
        let subscript = Msub {
            base: Box::<Element>::new(Element::Mi(beta)),
            subscript: Box::<Element>::new(Element::Mi(alpha)),
        };
        let whole = Msup {
            base: Box::<Element>::new(Element::Msub(subscript)),
            superscript: Box::<Element>::new(Element::Mn(number)),
        };
        let font_size = 100.0;

        let img = whole.render(font_size, 0);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[test]
    fn alpha_over_beta() {
        let alpha = Mi {
            identifier: "α".into(),
        };
        let beta = Mi {
            identifier: "β".into(),
        };
        let fraction = Mfrac {
            numer: Box::<Element>::new(Element::Mi(alpha)),
            denom: Box::<Element>::new(Element::Mi(beta)),
        };
        let font_size = 100.0;

        let img = fraction.render(font_size, 0);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }
}
