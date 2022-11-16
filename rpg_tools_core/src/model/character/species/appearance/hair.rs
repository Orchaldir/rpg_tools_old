use crate::model::character::appearance::hair::Hair;

/// Which [`Hair`]s are available for members of a [`Species`](crate::model::character::species::Species)?
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HairOption {
    NoHair,
    HasHair,
}

impl HairOption {
    /// Is the [`Hair`] valid for this option?
    ///
    /// ```
    ///# use rpg_tools_core::model::character::species::appearance::hair::HairOption::*;
    ///# use rpg_tools_core::model::character::appearance::hair::Hair;
    ///
    /// assert!(NoHair.is_valid(Hair::NoHair));
    /// assert!(!NoHair.is_valid(Hair::HasHair));
    ///
    /// assert!(!HasHair.is_valid(Hair::NoHair));
    /// assert!(HasHair.is_valid(Hair::HasHair));
    /// ```
    pub fn is_valid(&self, hair: Hair) -> bool {
        match self {
            HairOption::NoHair => hair == Hair::NoHair,
            HairOption::HasHair => hair == Hair::HasHair,
        }
    }
}
