use crate::*;
#[derive(Clone, Debug, trans::Trans, RustcEncodable)]
pub enum PlayerMessageGame {
    CustomDataMessage {
        data: CustomData,
    },
    ActionMessage {
        action: Versioned,
    },
}
