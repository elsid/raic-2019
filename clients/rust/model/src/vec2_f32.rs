use crate::*;
#[derive(Clone, Debug, trans::Trans, RustcEncodable)]
pub struct Vec2F32 {
    pub x: f32,
    pub y: f32,
}
