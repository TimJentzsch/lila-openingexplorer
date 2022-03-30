use serde::Deserialize;
use shakmaty::variant::Variant;

#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum LilaVariant {
    #[serde(alias = "antichess")]
    Antichess,
    #[serde(alias = "atomic")]
    Atomic,
    #[serde(alias = "chess960")]
    Chess960,
    #[serde(alias = "crazyhouse")]
    Crazyhouse,
    #[serde(alias = "fromPosition", alias = "From Position")]
    FromPosition,
    #[serde(alias = "horde")]
    Horde,
    #[serde(alias = "kingOfTheHill", alias = "King of the Hill")]
    KingOfTheHill,
    #[serde(alias = "racingKings", alias = "Racing Kings")]
    RacingKings,
    #[serde(alias = "chess", alias = "standard")]
    Standard,
    #[serde(alias = "threeCheck", alias = "Three-check")]
    ThreeCheck,
}

impl Default for LilaVariant {
    fn default() -> LilaVariant {
        LilaVariant::Standard
    }
}

impl From<LilaVariant> for Variant {
    fn from(variant: LilaVariant) -> Variant {
        match variant {
            LilaVariant::Standard | LilaVariant::Chess960 | LilaVariant::FromPosition => {
                Variant::Chess
            }
            LilaVariant::Antichess => Variant::Antichess,
            LilaVariant::Atomic => Variant::Atomic,
            LilaVariant::Crazyhouse => Variant::Crazyhouse,
            LilaVariant::Horde => Variant::Horde,
            LilaVariant::KingOfTheHill => Variant::KingOfTheHill,
            LilaVariant::RacingKings => Variant::RacingKings,
            LilaVariant::ThreeCheck => Variant::ThreeCheck,
        }
    }
}
