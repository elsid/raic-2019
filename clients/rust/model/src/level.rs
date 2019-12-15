use crate::*;
#[derive(Clone, Debug, trans::Trans, RustcEncodable)]
pub struct Level {
    pub tiles: Vec<Vec<Tile>>,
}
