use crate::*;
#[derive(Clone, Debug, trans::Trans, RustcEncodable)]
pub struct Vec2F64 {
    pub x: f64,
    pub y: f64,
}
