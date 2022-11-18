use crate::model::character::appearance::skin::Skin;

/// The available options of [`Skin`] for a [`Species`](crate::model::character::species::Species).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SkinOption {
    HasSkin,
    HasScales,
}

impl SkinOption {
    /// Is the [`Skin`] valid for this option?
    ///
    /// ```
    ///# use rpg_tools_core::model::character::species::appearance::skin::SkinOption::*;
    ///# use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
    ///
    /// assert!(HasSkin.is_valid(Skin::Skin(SkinColor::Tan)));
    /// assert!(!HasSkin.is_valid(Skin::Scales));
    ///
    /// assert!(!HasScales.is_valid(Skin::Skin(SkinColor::Dark)));
    /// assert!(HasScales.is_valid(Skin::Scales));
    /// ```
    pub fn is_valid(&self, skin: Skin) -> bool {
        match self {
            SkinOption::HasSkin => matches!(skin, Skin::Skin { .. }),
            SkinOption::HasScales => skin == Skin::Scales,
        }
    }
}
