use crate::*;
#[derive(Clone, Debug, trans::Trans, RustcEncodable)]
#[trans(magic = "43981")]
pub struct Versioned {
    pub inner: std::collections::HashMap<i32, UnitAction>,
}
