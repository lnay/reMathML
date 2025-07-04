#[allow(unused)]
#[allow(dead_code)]
use std::cmp::Ordering;

use crate::mml_types::{
    Element, Mfrac, Mi, Mn, Mo, Mphantom, Mroot, Mrow, Msqrt, Msub, Msup, Mtext,
};
use crate::mml_types::{mfrac, mi, mn, mo, mroot, mrow, msub, msup, mtext};
use crate::text_rendering::{TextRenderer, render_text};
use tiny_skia::{FillRule, IntRect, Paint, PathBuilder, Pixmap, PixmapPaint, Stroke, Transform};

pub struct RenderingPlan {
    /// callback to render element on the pixmap at the position (x, y).
    /// arguments: text_renderer, pixmap, x, y, font_size
    pub callback: Box<dyn Fn(&mut TextRenderer, &mut Pixmap, u32, u32) -> ()>,
    /// vertical position on the element where the horizontal line of the '+' symbol should be aligned with
    pub baseline: u32,
    pub width: u32,
    pub height: u32,
}

pub trait Render {
    fn plan_render(&self, text_renderer: &mut TextRenderer, font_size: f32) -> RenderingPlan;

    fn render(&self, text_renderer: &mut TextRenderer, font_size: f32) -> Pixmap {
        let RenderingPlan {
            callback,
            baseline,
            width,
            height,
        } = self.plan_render(text_renderer, font_size);
        let mut pixmap = Pixmap::new(width, height).unwrap();
        callback(text_renderer, &mut pixmap, 0, 0);

        pixmap
    }
}

impl Render for Mi {
    fn plan_render(&self, text_renderer: &mut TextRenderer, font_size: f32) -> RenderingPlan {
        text_renderer.plan_render_text(self.identifier.clone(), font_size)
    }
}

impl Render for Mtext {
    fn plan_render(&self, text_renderer: &mut TextRenderer, font_size: f32) -> RenderingPlan {
        text_renderer.plan_render_text(self.text.clone(), font_size)
    }
}

impl Render for Mn {
    fn plan_render(&self, text_renderer: &mut TextRenderer, font_size: f32) -> RenderingPlan {
        text_renderer.plan_render_text(self.number.clone(), font_size)
    }
}
impl Render for Mo {
    fn plan_render(&self, text_renderer: &mut TextRenderer, font_size: f32) -> RenderingPlan {
        let spacing = (font_size / 10.0).floor() as u32;
        let RenderingPlan {
            callback,
            baseline,
            width,
            height,
        } = text_renderer.plan_render_text(self.operator.clone(), font_size);
        let width = width + 2 * spacing;
        let callback =
            move |text_renderer: &mut TextRenderer, pixmap: &mut Pixmap, x: u32, y: u32| {
                callback(text_renderer, pixmap, x + spacing, y);
            };
        RenderingPlan {
            callback: Box::new(callback),
            baseline,
            width,
            height,
        }
    }
}
impl Render for Msup {
    fn plan_render(&self, text_renderer: &mut TextRenderer, font_size: f32) -> RenderingPlan {
        let paint = PixmapPaint::default();
        let transform = Transform::default();
        const SUPERSCRIPT_FONT_RATIO: f32 = 0.7;
        const SUPERSCRIPT_VERTICAL_OFFSET: f32 = 0.5;
        // let (base, y)
        let RenderingPlan {
            callback: base_callback,
            baseline: base_baseline,
            width: base_width,
            height: base_height,
        } = self.base.plan_render(text_renderer, font_size);
        // let (superscript, _)
        let RenderingPlan {
            callback: superscript_callback,
            baseline: superscript_baseline,
            width: superscript_width,
            height: superscript_height,
        } = self
            .superscript
            .plan_render(text_renderer, font_size * SUPERSCRIPT_FONT_RATIO);

        let width = base_width + superscript_width;
        let height = base_height.max(superscript_height * 2);
        let base_y_offset = 0.max(superscript_height - base_height / 2);
        let baseline = base_baseline + base_y_offset;

        let callback =
            move |text_renderer: &mut TextRenderer, pixmap: &mut Pixmap, x: u32, y: u32| {
                base_callback(text_renderer, pixmap, x, y + base_y_offset);
                superscript_callback(text_renderer, pixmap, x + base_width, y);
            };

        RenderingPlan {
            callback: Box::new(callback),
            baseline,
            width,
            height,
        }
    }
}
impl Render for Msub {
    fn plan_render(&self, text_renderer: &mut TextRenderer, font_size: f32) -> RenderingPlan {
        let paint = PixmapPaint::default();
        let transform = Transform::default();
        const SUBSCRIPT_FONT_RATIO: f32 = 0.7;
        const SUPERSCRIPT_VERTICAL_OFFSET: f32 = 0.5;
        let RenderingPlan {
            callback: base_callback,
            baseline: base_baseline,
            width: base_width,
            height: base_height,
        } = self.base.plan_render(text_renderer, font_size);
        // let (superscript, _)
        let RenderingPlan {
            callback: subscript_callback,
            baseline: subscript_baseline,
            width: subscript_width,
            height: subscript_height,
        } = self
            .subscript
            .plan_render(text_renderer, font_size * SUBSCRIPT_FONT_RATIO);

        let width = base_width + subscript_width;
        let height = base_height.max(2 * subscript_height);
        let mut pixmap = Pixmap::new(width, height).unwrap();

        let base_y_offset = 0.max(subscript_height - base_height / 2);
        let subscript_y_offset = base_y_offset + (base_height / 2);
        let baseline = base_baseline + base_y_offset;

        let callback =
            move |text_renderer: &mut TextRenderer, pixmap: &mut Pixmap, x: u32, y: u32| {
                base_callback(text_renderer, pixmap, x, y + base_y_offset);
                subscript_callback(
                    text_renderer,
                    pixmap,
                    x + base_width,
                    y + subscript_y_offset,
                );
            };

        RenderingPlan {
            callback: Box::new(callback),
            baseline,
            width,
            height,
        }
    }
}

impl Render for Mrow {
    fn plan_render(&self, text_renderer: &mut TextRenderer, font_size: f32) -> RenderingPlan {
        let paint = PixmapPaint::default();
        let transform = Transform::default();

        let children_render_plans = self
            .terms
            .iter()
            .map(|child| child.plan_render(text_renderer, font_size))
            .collect::<Vec<_>>();
        let baseline = children_render_plans
            .iter()
            .map(|child| child.baseline)
            .max()
            .unwrap();
        let below_baseline = children_render_plans
            .iter()
            .map(|child| child.height as i32 - baseline as i32)
            .max()
            .unwrap() as u32;
        let width = children_render_plans
            .iter()
            .map(|child| child.width)
            .sum::<u32>();
        let height = baseline + below_baseline;

        let callback = move |text_renderer: &mut TextRenderer,
                             pixmap: &mut Pixmap,
                             offset_x: u32,
                             offset_y: u32| {
            let mut offset_in_row = 0;
            children_render_plans.iter().for_each(|child| {
                let y = baseline - child.baseline;
                let child_callback = &child.callback;
                child_callback(
                    text_renderer,
                    pixmap,
                    offset_in_row + offset_x,
                    y + offset_y,
                );
                offset_in_row += child.width;
            });
        };

        RenderingPlan {
            callback: Box::new(callback),
            baseline,
            width,
            height,
        }
    }
}

impl Render for Mphantom {
    fn plan_render(&self, text_renderer: &mut TextRenderer, font_size: f32) -> RenderingPlan {
        let children_render_plans = self
            .terms
            .iter()
            .map(|child| child.plan_render(text_renderer, font_size))
            .collect::<Vec<_>>();
        let baseline = children_render_plans
            .iter()
            .map(|child| child.baseline)
            .max()
            .unwrap();
        let below_baseline = children_render_plans
            .iter()
            .map(|child| child.height as i32 - baseline as i32)
            .max()
            .unwrap() as u32;
        let width = children_render_plans
            .iter()
            .map(|child| child.width)
            .sum::<u32>();
        let height = baseline + below_baseline;

        let callback = move |text_renderer: &mut TextRenderer,
                             pixmap: &mut Pixmap,
                             offset_x: u32,
                             offset_y: u32| {};

        RenderingPlan {
            callback: Box::new(callback),
            baseline,
            width,
            height,
        }
    }
}

impl Render for Mfrac {
    fn plan_render(&self, text_renderer: &mut TextRenderer, font_size: f32) -> RenderingPlan {
        let paint = PixmapPaint::default();
        let transform = Transform::default();
        let line_width = (font_size / 25.0).ceil() as u32;
        let RenderingPlan {
            callback: numer_callback,
            baseline: numer_baseline,
            width: numer_width,
            height: numer_height,
        } = self.numerator.plan_render(text_renderer, font_size);
        let RenderingPlan {
            callback: denom_callback,
            baseline: denom_baseline,
            width: denom_width,
            height: denom_height,
        } = self.denominator.plan_render(text_renderer, font_size);

        let width = numer_width.max(denom_width);
        let term_height = numer_height.max(denom_height);
        let height = (2 * term_height + line_width);

        let (numerator_x_offset, denominator_x_offset) = match numer_width.cmp(&denom_width) {
            Ordering::Less => ((denom_width - numer_width) / 2, 0),
            Ordering::Equal => (0, 0),
            Ordering::Greater => (0, (numer_width - denom_width) / 2),
        };

        let baseline = term_height + line_width / 2;

        let callback = move |text_renderer: &mut TextRenderer,
                             pixmap: &mut Pixmap,
                             x_offset: u32,
                             y_offset: u32| {
            numer_callback(
                text_renderer,
                pixmap,
                numerator_x_offset + x_offset,
                (term_height - numer_height) + y_offset,
            );
            denom_callback(
                text_renderer,
                pixmap,
                denominator_x_offset + x_offset,
                (term_height + line_width) + y_offset,
            );
            pixmap.fill_rect(
                IntRect::from_xywh(
                    x_offset as i32,
                    (y_offset + term_height) as i32,
                    width,
                    line_width,
                )
                .unwrap()
                .to_rect(),
                &Paint::default(),
                transform,
                None,
            );
        };

        RenderingPlan {
            callback: Box::new(callback),
            baseline,
            width,
            height,
        }
    }
}

struct _Mroot<'a> {
    base: &'a Element,
    index: Option<&'a Element>,
}

impl Render for _Mroot<'_> {
    fn plan_render(&self, text_renderer: &mut TextRenderer, font_size: f32) -> RenderingPlan {
        let paint = Paint::default();
        let pixmappaint = PixmapPaint::default();
        let transform = Transform::default();
        let line_width = (font_size / 25.0).ceil() as u32;

        let RenderingPlan {
            callback: inner_callback,
            baseline: inner_baseline,
            width: inner_width,
            height: inner_height,
        } = self.base.plan_render(text_renderer, font_size);

        let baseline = inner_baseline + line_width;
        let width = inner_width + inner_height / 2;
        let height = inner_height + 3 * line_width;

        let callback =
            move |text_renderer: &mut TextRenderer, pixmap: &mut Pixmap, x: u32, y: u32| {
                inner_callback(
                    text_renderer,
                    pixmap,
                    x + inner_height / 2,
                    y + 2 * line_width,
                );

                let x = x as f32;
                let y = y as f32;
                let mut root_linepath = PathBuilder::new();
                let mut stroke = Stroke::default();
                stroke.width = line_width as f32;
                root_linepath.move_to(x + width as f32, y + 3. * line_width as f32 / 2.);
                root_linepath.line_to(
                    x + inner_height as f32 / 2.,
                    y + 3. * line_width as f32 / 2.,
                );
                root_linepath.line_to(
                    x + inner_height as f32 / 4.,
                    y + height as f32 - line_width as f32,
                );
                root_linepath.line_to(
                    x + height as f32 / 9.,
                    y + 2. * (height as f32 - line_width as f32) / 3., // + line_width?
                );
                root_linepath.line_to(x, y + 7. * (height as f32 - line_width as f32) / 9.); // + linewidth?
                // root_linepath.close();
                pixmap.stroke_path(
                    &root_linepath.finish().unwrap(),
                    &paint,
                    &stroke,
                    Transform::identity(),
                    None,
                );
            };

        RenderingPlan {
            callback: Box::new(callback),
            baseline,
            width,
            height,
        }
    }
}
impl Render for Mroot {
    fn plan_render(&self, text_renderer: &mut TextRenderer, font_size: f32) -> RenderingPlan {
        _Mroot {
            base: &self.base,
            index: self.index.as_ref().map(|x| &(**x)), // is there a cleaner way to do this?
        }
        .plan_render(text_renderer, font_size)
    }
}
impl Render for Msqrt {
    fn plan_render(&self, text_renderer: &mut TextRenderer, font_size: f32) -> RenderingPlan {
        _Mroot {
            base: &self.term,
            index: None,
        }
        .plan_render(text_renderer, font_size)
    }
}

impl Render for Element {
    fn plan_render(&self, text_renderer: &mut TextRenderer, font_size: f32) -> RenderingPlan {
        match self {
            Element::Mi(mi) => mi.plan_render(text_renderer, font_size),
            Element::Mn(mn) => mn.plan_render(text_renderer, font_size),
            Element::Mo(mo) => mo.plan_render(text_renderer, font_size),
            Element::Mtext(mtext) => mtext.plan_render(text_renderer, font_size),
            Element::Msup(msup) => msup.plan_render(text_renderer, font_size),
            Element::Msub(msub) => msub.plan_render(text_renderer, font_size),
            Element::Mfrac(mfrac) => mfrac.plan_render(text_renderer, font_size),
            Element::Mroot(mroot) => mroot.plan_render(text_renderer, font_size),
            Element::Msqrt(msqrt) => msqrt.plan_render(text_renderer, font_size),
            Element::Mrow(mrow) => mrow.plan_render(text_renderer, font_size),
            Element::Mphantom(mphantom) => mphantom.plan_render(text_renderer, font_size),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use crate::mml_types::mphantom;

    use super::*;
    use function_name::named;
    use parley::swash::text;
    use test::Bencher;

    #[named]
    #[test]
    fn single_char() {
        let whole = mi("β");
        let font_size = 100.0;

        let img = whole.render(&mut TextRenderer::new(), font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[test]
    fn beta_squared() {
        let whole = msup(mi("β"), mn("2"));
        let font_size = 100.0;

        let img = whole.render(&mut TextRenderer::new(), font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[test]
    fn phantom_plus() {
        let whole = mrow(vec![mi("β"), mphantom(vec![mo("+")]), mn("2")]);
        let font_size = 100.0;

        let img = whole.render(&mut TextRenderer::new(), font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[test]
    fn beta_sub_alpha_sup_2() {
        let whole = msup(msub(mi("β"), mi("α")), mn("2"));
        let font_size = 100.0;

        let img = whole.render(&mut TextRenderer::new(), font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[test]
    fn alpha_over_beta() {
        let fraction = mfrac(mi("α"), mi("β"));
        let font_size = 100.0;

        let img = fraction.render(&mut TextRenderer::new(), font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[test]
    fn half_alpha_n() {
        let fraction = mfrac(msub(mi("α"), mn("n")), mn("2"));
        let font_size = 100.0;

        let img = fraction.render(&mut TextRenderer::new(), font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[test]
    fn alpha_plus_beta() {
        let row = mrow(vec![mi("α"), mo("+"), mi("β")]);
        let font_size = 100.0;

        let img = row.render(&mut TextRenderer::new(), font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[test]
    fn x_and_a_half() {
        let row = mrow(vec![mi("x"), mo("+"), mfrac(mn("1"), mn("2"))]);
        let font_size = 100.0;

        let img = row.render(&mut TextRenderer::new(), font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[test]
    fn sqrt2() {
        let sqrt_2 = mroot(mn("2"), None);
        let font_size = 100.0;

        let img = sqrt_2.render(&mut TextRenderer::new(), font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[test]
    fn sqrtsqrt2() {
        let sqrt_2 = mroot(mroot(mn("2"), None), None);
        let font_size = 100.0;

        let img = sqrt_2.render(&mut TextRenderer::new(), font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[test]
    fn discriminant() {
        let font_size = 100.0;
        let img = mrow(vec![
            mtext("roots"),
            mo("="),
            mfrac(
                mrow(vec![
                    mo("−"),
                    mi("b"),
                    mo("±"),
                    mroot(
                        mrow(vec![
                            msup(mi("b"), mn("2")),
                            mo("−"),
                            mn("4"),
                            mi("a"),
                            mi("c"),
                        ]),
                        None,
                    ),
                ]),
                mrow(vec![mn("2"), mi("a")]),
            ),
        ])
        .render(&mut TextRenderer::new(), font_size);

        img.save_png(format!("examples/{}.png", function_name!()))
            .unwrap();
    }

    #[named]
    #[bench]
    fn discriminant_bench(b: &mut Bencher) {
        let font_size = 100.0;
        let expression = mrow(vec![
            mtext("roots"),
            mo("="),
            mfrac(
                mrow(vec![
                    mo("−"),
                    mi("b"),
                    mo("±"),
                    mroot(
                        mrow(vec![
                            msup(mi("b"), mn("2")),
                            mo("−"),
                            mn("4"),
                            mi("a"),
                            mi("c"),
                        ]),
                        None,
                    ),
                ]),
                mrow(vec![mn("2"), mi("a")]),
            ),
        ]);
        let mut text_renderer = TextRenderer::new();

        b.iter(|| expression.render(&mut text_renderer, font_size));
    }
}
