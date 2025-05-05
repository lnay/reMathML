use std::cmp::Ordering;

#[allow(unused)]
#[allow(dead_code)]
use crate::mml_types::{Mfrac, Mi, Mn, Mo, Msub, Msup, mfrac, mi, mn, mo, msub, msup};
use crate::text_rendering::render_text;
use tiny_skia::{IntRect, Paint, Pixmap, PixmapPaint, Transform};

pub trait Render {
    fn pixmap_with_baseline(&self, font_size: f32) -> (Pixmap, u32);
    fn render(&self, font_size: f32) -> Pixmap {
        let (pixmap, _) = self.pixmap_with_baseline(font_size);
        pixmap
    }
}

impl Render for Mi {
    fn pixmap_with_baseline(&self, font_size: f32) -> (Pixmap, u32) {
        let pixmap = render_text(self.identifier.clone(), font_size);
        let y = 2 * pixmap.height() / 3;
        (pixmap, y)
    }
}

impl Render for Mn {
    fn pixmap_with_baseline(&self, font_size: f32) -> (Pixmap, u32) {
        let pixmap = render_text(self.number.clone(), font_size);
        let y = 2 * pixmap.height() / 3;
        (pixmap, y)
    }
}
impl Render for Mo {
    fn pixmap_with_baseline(&self, font_size: f32) -> (Pixmap, u32) {
        let pixmap = render_text(self.operator.clone(), font_size);
        let y = 2 * pixmap.height() / 3;
        (pixmap, y)
    }
}
impl Render for Msup<'_> {
    fn pixmap_with_baseline(&self, font_size: f32) -> (Pixmap, u32) {
        let paint = PixmapPaint::default();
        let transform = Transform::default();
        const SUPERSCRIPT_FONT_RATIO: f32 = 0.7;
        const SUPERSCRIPT_VERTICAL_OFFSET: f32 = 0.5;
        let (base, y) = self.base.pixmap_with_baseline(font_size);
        let (superscript, _) = self
            .superscript
            .pixmap_with_baseline(font_size * SUPERSCRIPT_FONT_RATIO);
        let width = base.width() + superscript.width();
        let height = base.height().max(superscript.height() * 2);
        let base_y_offset = 0.max(superscript.height() - base.height() / 2) as i32;
        let mut pixmap = Pixmap::new(width, height).unwrap();
        let base_width = base.width() as i32;
        pixmap.draw_pixmap(0, base_y_offset, base.as_ref(), &paint, transform, None);
        pixmap.draw_pixmap(base_width, 0, superscript.as_ref(), &paint, transform, None);
        (pixmap, y + base_y_offset as u32)
    }
}
impl Render for Msub<'_> {
    fn pixmap_with_baseline(&self, font_size: f32) -> (Pixmap, u32) {
        let paint = PixmapPaint::default();
        let transform = Transform::default();
        const SUBSCRIPT_FONT_RATIO: f32 = 0.7;
        const SUPERSCRIPT_VERTICAL_OFFSET: f32 = 0.5;
        let (base, y) = self.base.pixmap_with_baseline(font_size);
        let (subscript, _) = self
            .subscript
            .pixmap_with_baseline(font_size * SUBSCRIPT_FONT_RATIO);
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
        (pixmap, y + base_y_offset as u32)
    }
}
impl Render for Mfrac<'_> {
    fn pixmap_with_baseline(&self, font_size: f32) -> (Pixmap, u32) {
        let paint = PixmapPaint::default();
        let transform = Transform::default();
        let line_width = (font_size / 20.0).ceil() as u32;
        let (numerator, _) = self.numer.pixmap_with_baseline(font_size);
        let (denominator, _) = self.denom.pixmap_with_baseline(font_size);
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
        (pixmap, term_height + line_width / 2)
    }
}
// impl Render for Element {
//     fn pixmap_with_baseline(&self, font_size: f32) -> (Pixmap, u32) {
//         match self {
//             Element::Mi(mi) => mi.pixmap_with_baseline(font_size),
//             Element::Mn(mn) => mn.pixmap_with_baseline(font_size),
//             Element::Mo(mo) => mo.pixmap_with_baseline(font_size),
//             Element::Msup(msup) => msup.pixmap_with_baseline(font_size),
//             Element::Msub(msub) => msub.pixmap_with_baseline(font_size),
//             Element::Mfrac(mfrac) => mfrac.pixmap_with_baseline(font_size),
//             _ => unimplemented!("Render not implemented for all MathML node types"),
//         }
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;
    use function_name::named;

    #[named]
    #[test]
    fn beta_sub_alpha_sup_2() {
        let alpha = mi("α");
        let beta = mi("β");
        let number = mn("2");
        let subscript = msub(&beta, &alpha);
        let whole = msup(&subscript, &number);
        let font_size = 100.0;

        let img = whole.render(font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[test]
    fn alpha_over_beta() {
        let alpha = mi("α");
        let beta = mi("β");
        let fraction = mfrac(&alpha, &beta);
        let font_size = 100.0;

        let img = fraction.render(font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[test]
    fn half_alpha_n() {
        let alpha = mi("α");
        let n = mn("n");
        let two = mn("2");
        let alpha_n = msub(&alpha, &n);
        let fraction = mfrac(&alpha_n, &two);
        let font_size = 100.0;

        let img = fraction.render(font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }
}
