use crate::*;
#[derive(Clone, Debug, PartialEq, Eq, Hash, trans::Trans, RustcEncodable)]
pub enum WeaponType {
    Pistol,
    AssaultRifle,
    RocketLauncher,
}
