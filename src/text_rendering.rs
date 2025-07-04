// Copyright 2024 the Parley Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A simple example that lays out some text using Parley, extracts outlines using Skrifa and
//! then paints those outlines using Tiny-Skia.
//!
//! Note: Emoji rendering is not currently implemented in this example. See the swash example
//! if you need emoji rendering.

use crate::render::RenderingPlan;
use parley::{
    FontContext, FontFamily, GlyphRun, Layout, LayoutContext, PositionedLayoutItem, StyleProperty,
};
use skrifa::{
    GlyphId, MetadataProvider, OutlineGlyph,
    instance::{LocationRef, NormalizedCoord, Size},
    outline::{DrawSettings, OutlinePen},
    raw::FontRef as ReadFontsRef,
};
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, PixmapMut, Transform};

#[derive(Clone, Copy, Debug, PartialEq)]
struct ColorBrush {
    color: Color,
}

impl Default for ColorBrush {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
        }
    }
}

pub struct TextRenderer {
    font_cx: FontContext,
    layout_cx: LayoutContext<ColorBrush>,
}

impl TextRenderer {
    pub fn new() -> Self {
        // Create a FontContext, LayoutContext
        //
        // These are both intended to be constructed rarely (perhaps even once per app (or once per thread))
        // and provide caches and scratch space to avoid allocations
        let font_cx = FontContext::new();
        let layout_cx = LayoutContext::new();
        Self { font_cx, layout_cx }
    }
    pub fn render_text(&mut self, text: String, font_size: f32) -> Pixmap {
        // The display scale for HiDPI rendering
        let display_scale = 1.0;

        // Create a RangedBuilder
        let mut builder = self
            .layout_cx
            .ranged_builder(&mut self.font_cx, &text, display_scale);
        // let mut builder = layout_cx.ranged_builder(font_cx, &text, display_scale);

        // Set default text colour styles (set foreground text color)
        builder.push_default(StyleProperty::Brush(ColorBrush {
            color: Color::BLACK,
        }));

        // Set default font family
        // builder.push_default(StyleProperty::FontStyle(parley::FontStyle::Italic));
        // builder.push_default(GenericFamily::Math);
        // builder.push_default(FontFamily::parse("TeX Gyre Pagella Math").unwrap());
        // builder.push_default(FontFamily::parse("Libertinus Math").unwrap());
        // builder.push_default(FontFamily::parse("Tex Gyre Termes Math").unwrap());
        builder.push_default(FontFamily::parse("TexMaths Symbols").unwrap());
        builder.push_default(StyleProperty::FontSize(font_size));

        // Build the builder into a Layout
        let mut layout: Layout<ColorBrush> = builder.build(&text);

        // Perform layout (including bidi resolution and shaping) with start alignment
        layout.break_all_lines(None);
        let width = layout.width().ceil() as u32;
        let height = layout.height().ceil() as u32;

        // Create TinySkia Pixmap
        let mut img = Pixmap::new(width, height).unwrap();

        // Fill background color
        img.fill(Color::WHITE);

        // Wrap Pixmap in a type that implements skrifa::OutlinePen
        let mut pen = TinySkiaPen::new(img.as_mut());

        // Render each glyph run
        for line in layout.lines() {
            for item in line.items() {
                match item {
                    PositionedLayoutItem::GlyphRun(glyph_run) => {
                        render_glyph_run(&glyph_run, &mut pen, 0, 0, 0);
                    }
                    // PositionedLayoutItem::InlineBox(inline_box) => {
                    //     pen.set_origin(inline_box.x, inline_box.y);
                    //     pen.set_color(foreground_color);
                    //     pen.fill_rect(inline_box.width, inline_box.height);
                    // }
                    _ => {}
                }
            }
        }
        img
    }
    pub fn plan_render_text(&mut self, text: String, font_size: f32) -> RenderingPlan {
        // The display scale for HiDPI rendering
        let display_scale = 1.0;

        // Create a RangedBuilder
        let mut builder = self
            .layout_cx
            .ranged_builder(&mut self.font_cx, &text, display_scale);
        // let mut builder = layout_cx.ranged_builder(font_cx, &text, display_scale);

        // Set default text colour styles (set foreground text color)
        builder.push_default(StyleProperty::Brush(ColorBrush {
            color: Color::BLACK,
        }));

        // Set default font family
        // builder.push_default(StyleProperty::FontStyle(parley::FontStyle::Italic));
        // builder.push_default(GenericFamily::Math);
        // builder.push_default(FontFamily::parse("TeX Gyre Pagella Math").unwrap());
        // builder.push_default(FontFamily::parse("Libertinus Math").unwrap());
        // builder.push_default(FontFamily::parse("Tex Gyre Termes Math").unwrap());
        builder.push_default(FontFamily::parse("TexMaths Symbols").unwrap());
        builder.push_default(StyleProperty::FontSize(font_size));

        // Build the builder into a Layout
        let mut layout: Layout<ColorBrush> = builder.build(&text);

        // Perform layout (including bidi resolution and shaping) with start alignment
        layout.break_all_lines(None);
        let width = layout.width().ceil() as u32;
        let height = layout.height().ceil() as u32;
        let baseline = height / 2;

        let callback = move |text_rendered: &mut TextRenderer, img: &mut Pixmap, x: u32, y: u32| {
            // Fill background color
            // img.fill(Color::WHITE);

            // Wrap Pixmap in a type that implements skrifa::OutlinePen
            let mut pen = TinySkiaPen::new(img.as_mut());

            // Render each glyph run
            for line in layout.lines() {
                for item in line.items() {
                    match item {
                        PositionedLayoutItem::GlyphRun(glyph_run) => {
                            render_glyph_run(&glyph_run, &mut pen, x, y, 0);
                        }
                        // PositionedLayoutItem::InlineBox(inline_box) => {
                        //     pen.set_origin(inline_box.x, inline_box.y);
                        //     pen.set_color(foreground_color);
                        //     pen.fill_rect(inline_box.width, inline_box.height);
                        // }
                        _ => {}
                    }
                }
            }
        };

        RenderingPlan {
            callback: Box::new(callback),
            /// vertical position on the element where the horizontal line of the '+' symbol should be aligned with
            baseline,
            width,
            height,
        }
    }
}

pub fn render_text(text: String, font_size: f32) -> Pixmap {
    // The display scale for HiDPI rendering
    let display_scale = 1.0;

    // Colours for rendering
    let foreground_color = Color::BLACK;
    let background_color = Color::WHITE;

    // Create a FontContext, LayoutContext
    //
    // These are both intended to be constructed rarely (perhaps even once per app (or once per thread))
    // and provide caches and scratch space to avoid allocations
    let mut font_cx = FontContext::new();
    let mut layout_cx = LayoutContext::new();

    // Create a RangedBuilder
    let mut builder = layout_cx.ranged_builder(&mut font_cx, &text, display_scale);

    // Set default text colour styles (set foreground text color)
    let foreground_brush = ColorBrush {
        color: Color::BLACK,
    };
    let brush_style = StyleProperty::Brush(foreground_brush);
    builder.push_default(brush_style);

    // Set default font family
    // builder.push_default(StyleProperty::FontStyle(parley::FontStyle::Italic));
    // builder.push_default(GenericFamily::Math);
    // builder.push_default(FontFamily::parse("TeX Gyre Pagella Math").unwrap());
    // builder.push_default(FontFamily::parse("Libertinus Math").unwrap());
    // builder.push_default(FontFamily::parse("Tex Gyre Termes Math").unwrap());
    builder.push_default(FontFamily::parse("TexMaths Symbols").unwrap());
    builder.push_default(StyleProperty::FontSize(font_size));

    // Build the builder into a Layout
    let mut layout: Layout<ColorBrush> = builder.build(&text);

    // Perform layout (including bidi resolution and shaping) with start alignment
    layout.break_all_lines(None);
    let width = layout.width().ceil() as u32;
    let height = layout.height().ceil() as u32;

    // Create TinySkia Pixmap
    let mut img = Pixmap::new(width, height).unwrap();

    // Fill background color
    img.fill(background_color);

    // Wrap Pixmap in a type that implements skrifa::OutlinePen
    let mut pen = TinySkiaPen::new(img.as_mut());

    // Render each glyph run
    for line in layout.lines() {
        for item in line.items() {
            match item {
                PositionedLayoutItem::GlyphRun(glyph_run) => {
                    render_glyph_run(&glyph_run, &mut pen, 0, 0, 0);
                }
                // PositionedLayoutItem::InlineBox(inline_box) => {
                //     pen.set_origin(inline_box.x, inline_box.y);
                //     pen.set_color(foreground_color);
                //     pen.fill_rect(inline_box.width, inline_box.height);
                // }
                _ => {}
            }
        }
    }
    img
}

fn render_glyph_run(
    glyph_run: &GlyphRun<'_, ColorBrush>,
    pen: &mut TinySkiaPen<'_>,
    offset_x: u32,
    offset_y: u32,
    padding: u32,
) {
    // Resolve properties of the GlyphRun
    let mut run_x = glyph_run.offset();
    let run_y = glyph_run.baseline();
    let style = glyph_run.style();
    let brush = style.brush;

    // Get the "Run" from the "GlyphRun"
    let run = glyph_run.run();

    // Resolve properties of the Run
    let font = run.font();
    let font_size = run.font_size();

    let normalized_coords = run
        .normalized_coords()
        .iter()
        .map(|coord| NormalizedCoord::from_bits(*coord))
        .collect::<Vec<_>>();

    // Get glyph outlines using Skrifa. This can be cached in production code.
    let font_collection_ref = font.data.as_ref();
    let font_ref = ReadFontsRef::from_index(font_collection_ref, font.index).unwrap();
    let outlines = font_ref.outline_glyphs();

    // Iterates over the glyphs in the GlyphRun
    for glyph in glyph_run.glyphs() {
        let glyph_x = run_x + glyph.x + padding as f32 + offset_x as f32;
        let glyph_y = run_y - glyph.y + padding as f32 + offset_y as f32;
        run_x += glyph.advance;

        let glyph_id = GlyphId::from(glyph.id);
        if let Some(glyph_outline) = outlines.get(glyph_id) {
            pen.set_origin(glyph_x, glyph_y);
            pen.set_color(brush.color);
            pen.draw_glyph(&glyph_outline, font_size, &normalized_coords);
        }
    }

    // Draw decorations: underline & strikethrough
    // let style = glyph_run.style();
    // let run_metrics = run.metrics();
    // if let Some(decoration) = &style.underline {
    //     let offset = decoration.offset.unwrap_or(run_metrics.underline_offset);
    //     let size = decoration.size.unwrap_or(run_metrics.underline_size);
    //     render_decoration(pen, glyph_run, decoration.brush, offset, size, padding);
    // }
    // if let Some(decoration) = &style.strikethrough {
    //     let offset = decoration
    //         .offset
    //         .unwrap_or(run_metrics.strikethrough_offset);
    //     let size = decoration.size.unwrap_or(run_metrics.strikethrough_size);
    //     render_decoration(pen, glyph_run, decoration.brush, offset, size, padding);
    // }
}

// fn render_decoration(
//     pen: &mut TinySkiaPen<'_>,
//     glyph_run: &GlyphRun<'_, ColorBrush>,
//     brush: ColorBrush,
//     offset: f32,
//     width: f32,
//     padding: u32,
// ) {
//     let y = glyph_run.baseline() - offset + padding as f32;
//     let x = glyph_run.offset() + padding as f32;
//     pen.set_color(brush.color);
//     pen.set_origin(x, y);
//     pen.fill_rect(glyph_run.advance(), width);
// }

struct TinySkiaPen<'a> {
    pub(crate) pixmap: PixmapMut<'a>,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) paint: Paint<'static>,
    pub(crate) open_path: PathBuilder,
}

impl TinySkiaPen<'_> {
    pub(crate) fn new(pixmap: PixmapMut<'_>) -> TinySkiaPen<'_> {
        TinySkiaPen {
            pixmap,
            x: 0.0,
            y: 0.0,
            paint: Paint::default(),
            open_path: PathBuilder::new(),
        }
    }

    pub(crate) fn set_origin(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub(crate) fn set_color(&mut self, color: Color) {
        self.paint.set_color(color);
    }

    // pub(crate) fn fill_rect(&mut self, width: f32, height: f32) {
    //     let rect = Rect::from_xywh(self.x, self.y, width, height).unwrap();
    //     self.pixmap
    //         .fill_rect(rect, &self.paint, Transform::identity(), None);
    // }

    pub(crate) fn draw_glyph(
        &mut self,
        glyph: &OutlineGlyph<'_>,
        size: f32,
        normalized_coords: &[NormalizedCoord],
    ) {
        let location_ref = LocationRef::new(normalized_coords);
        let settings = DrawSettings::unhinted(Size::new(size), location_ref);
        glyph.draw(settings, self).unwrap();

        let builder = core::mem::replace(&mut self.open_path, PathBuilder::new());
        if let Some(path) = builder.finish() {
            self.pixmap.fill_path(
                &path,
                &self.paint,
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        }
    }
}

impl OutlinePen for TinySkiaPen<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.open_path.move_to(self.x + x, self.y - y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.open_path.line_to(self.x + x, self.y - y);
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.open_path
            .quad_to(self.x + cx0, self.y - cy0, self.x + x, self.y - y);
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.open_path.cubic_to(
            self.x + cx0,
            self.y - cy0,
            self.x + cx1,
            self.y - cy1,
            self.x + x,
            self.y - y,
        );
    }

    fn close(&mut self) {
        self.open_path.close();
    }
}
