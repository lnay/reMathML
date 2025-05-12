use std::cmp::Ordering;

#[allow(unused)]
#[allow(dead_code)]
use crate::mml_types::{
    Mfrac, Mi, Mn, Mo, Mroot, Mrow, Msub, Msup, Mtext, mfrac, mi, mn, mo, mroot, mrow, msub, msup,
    mtext,
};
use crate::text_rendering::render_text;
use tiny_skia::{FillRule, IntRect, Paint, PathBuilder, Pixmap, PixmapPaint, Stroke, Transform};

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
        let y = pixmap.height() / 2;
        (pixmap, y)
    }
}

impl Render for Mtext {
    fn pixmap_with_baseline(&self, font_size: f32) -> (Pixmap, u32) {
        let pixmap = render_text(self.text.clone(), font_size);
        let y = pixmap.height() / 2;
        (pixmap, y)
    }
}

impl Render for Mn {
    fn pixmap_with_baseline(&self, font_size: f32) -> (Pixmap, u32) {
        let pixmap = render_text(self.number.clone(), font_size);
        let y = pixmap.height() / 2;
        (pixmap, y)
    }
}
impl Render for Mo {
    fn pixmap_with_baseline(&self, font_size: f32) -> (Pixmap, u32) {
        let pixmap = render_text(self.operator.clone(), font_size);
        let y = pixmap.height() / 2;
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
impl Render for Mrow<'_> {
    fn pixmap_with_baseline(&self, font_size: f32) -> (Pixmap, u32) {
        let paint = PixmapPaint::default();
        let transform = Transform::default();
        let spacing = (font_size / 10.0).floor() as u32;

        let rendered_children = self
            .children
            .iter()
            .map(|child| child.pixmap_with_baseline(font_size))
            .collect::<Vec<_>>();
        let baseline = rendered_children.iter().map(|child| child.1).max().unwrap();
        let below_baseline = rendered_children
            .iter()
            .map(|child| child.0.height() as i32 - baseline as i32)
            .max()
            .unwrap() as u32;
        let width = rendered_children
            .iter()
            .map(|child| child.0.width())
            .sum::<u32>()
            + spacing * (rendered_children.len() - 1) as u32;
        let height = baseline + below_baseline;
        let mut pixmap = Pixmap::new(width, height).unwrap();

        let mut x = 0;
        rendered_children.iter().for_each(|child| {
            let (child_pixmap, child_baseline) = child;
            let child_width = child_pixmap.width();
            let y = baseline - child_baseline;
            pixmap.draw_pixmap(
                x as i32,
                y as i32,
                child_pixmap.as_ref(),
                &paint,
                transform,
                None,
            );
            x += child_width;
            x += spacing;
        });
        (pixmap, baseline)
    }
}
impl Render for Mfrac<'_> {
    fn pixmap_with_baseline(&self, font_size: f32) -> (Pixmap, u32) {
        let paint = PixmapPaint::default();
        let transform = Transform::default();
        let line_width = (font_size / 25.0).ceil() as u32;
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
impl Render for Mroot<'_> {
    fn pixmap_with_baseline(&self, font_size: f32) -> (Pixmap, u32) {
        let paint = Paint::default();
        let pixmappaint = PixmapPaint::default();
        let transform = Transform::default();
        let line_width = (font_size / 25.0).ceil() as u32;

        let (inner, inner_baseline) = self.base.pixmap_with_baseline(font_size);

        let baseline = inner_baseline + line_width;
        let width = inner.width() + inner.height() / 2;
        let height = inner.height() + 2 * line_width;

        let mut pixmap = Pixmap::new(width, height).unwrap();
        pixmap.draw_pixmap(
            inner.height() as i32 / 2,
            line_width as i32,
            inner.as_ref(),
            &pixmappaint,
            transform,
            None,
        );
        let mut root_linepath = PathBuilder::new();
        let mut stroke = Stroke::default();
        stroke.width = line_width as f32;
        root_linepath.move_to(width as f32, line_width as f32 / 2.);
        root_linepath.line_to(inner.height() as f32 / 2., line_width as f32 / 2.);
        root_linepath.line_to(
            inner.height() as f32 / 4.,
            height as f32 - line_width as f32 / 2.,
        );
        root_linepath.line_to(
            height as f32 / 9.,
            2. * (height as f32 - line_width as f32) / 3.,
        );
        root_linepath.line_to(0., 7. * (height as f32 - line_width as f32) / 9.);
        // root_linepath.close();
        pixmap.stroke_path(
            &root_linepath.finish().unwrap(),
            &paint,
            &stroke,
            Transform::identity(),
            None,
        );
        (pixmap, baseline)
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

    #[named]
    #[test]
    fn alpha_plus_beta() {
        let alpha = mi("α");
        let beta = mi("β");
        let plus = mo("+");
        let row = mrow(vec![&alpha, &plus, &beta]);
        let font_size = 100.0;

        let img = row.render(font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[test]
    fn x_and_a_half() {
        let x = mi("x");
        let plus = mo("+");
        let one = mn("1");
        let two = mn("2");
        let half = mfrac(&one, &two);
        let row = mrow(vec![&x, &plus, &half]);
        let font_size = 100.0;

        let img = row.render(font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[test]
    fn sqrt2() {
        let two = mn("2");
        let sqrt_2 = mroot(&two, None);
        let font_size = 100.0;

        let img = sqrt_2.render(font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[test]
    fn discriminant() {
        let font_size = 100.0;
        let img = mrow(vec![
            &mtext("roots"),
            &mo("="),
            &mfrac(
                &mrow(vec![
                    &mo("−"),
                    &mi("b"),
                    &mo("±"),
                    &mroot(
                        &mrow(vec![
                            &msup(&mi("b"), &mn("2")),
                            &mo("−"),
                            &mn("4"),
                            &mi("a"),
                            &mi("c"),
                        ]),
                        None,
                    ),
                ]),
                &mrow(vec![&mn("2"), &mi("a")]),
            ),
        ])
        .render(font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }
}
