use crate::*;
#[derive(Clone, Debug, PartialEq, Eq, Hash, trans::Trans, RustcEncodable)]
pub enum MineState {
    Preparing,
    Idle,
    Triggered,
    Exploded,
}
