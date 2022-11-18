use crate::model::character::appearance::hair::{HairColor, HairLength};

/// The beard of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Beard {
    NoBeard,
    Beard { color: HairColor, style: BeardStyle },
}

impl Beard {
    pub fn new_beard(color: HairColor, style: BeardStyle) -> Self {
        Self::Beard { color, style }
    }
}

impl Default for Beard {
    fn default() -> Self {
        Beard::NoBeard
    }
}

/// The beard style of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BeardStyle {
    CleanShaven,
    FullBeard,
    Goatee,
    Imperial,
    LongBeard(HairLength),
    Mustache,
    MuttonChops,
    Stubble,
    VanDyke,
}
