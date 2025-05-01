#![expect(
    clippy::cast_possible_truncation,
    clippy::shadow_unrelated,
    reason = "Deferred"
)]
#[allow(dead_code)]
#[allow(unused)]
pub mod mml_types;
pub mod render;
pub mod text_rendering;
