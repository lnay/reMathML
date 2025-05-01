#![expect(
    clippy::cast_possible_truncation,
    clippy::shadow_unrelated,
    reason = "Deferred"
)]

use remml::mml_types::{Element, Mi, Mn, Msub, Msup};
use remml::render::Render;

fn main() {
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

    img.save_png("examples/tiny_skia_render.png").unwrap();
}
