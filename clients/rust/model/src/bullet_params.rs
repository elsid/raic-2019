use crate::*;
#[derive(Clone, Debug, trans::Trans, RustcEncodable)]
pub struct BulletParams {
    pub speed: f64,
    pub size: f64,
    pub damage: i32,
}
