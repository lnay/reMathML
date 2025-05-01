#![expect(
    clippy::cast_possible_truncation,
    clippy::shadow_unrelated,
    reason = "Deferred"
)]
mod text_rendering;

fn main() {
    // The text we are going to style and lay out
    let text = String::from("αB");
    let font_size = 16.0;

    let img = text_rendering::render_text(text, font_size);

    img.save_png("_output/tiny_skia_render.png").unwrap();
}
