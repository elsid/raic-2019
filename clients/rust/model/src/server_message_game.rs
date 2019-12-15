use crate::*;
#[derive(Clone, Debug, trans::Trans, RustcEncodable)]
pub struct ServerMessageGame {
    pub player_view: Option<PlayerView>,
}
