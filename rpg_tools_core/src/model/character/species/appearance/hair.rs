use crate::model::character::appearance::hair::Hair;

/// Which [`Hair`]s are available for members of a [`Species`](crate::model::character::species::Species)?
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HairOption {
    NoHair,
    NormalHair,
    SnakeHair,
}

impl HairOption {
    /// Is the [`Hair`] valid for this option?
    ///
    /// ```
    ///# use rpg_tools_core::model::character::species::appearance::hair::HairOption::*;
    ///# use rpg_tools_core::model::character::appearance::hair::{Hair, HairStyle};
    ///
    /// assert!(NoHair.is_valid(Hair::NoHair));
    /// assert!(!NoHair.is_valid(Hair::NormalHair{ style: HairStyle::Bun }));
    /// assert!(!NoHair.is_valid(Hair::SnakeHair));
    ///
    /// assert!(!NormalHair.is_valid(Hair::NoHair));
    /// assert!(NormalHair.is_valid(Hair::NormalHair{ style: HairStyle::Bun }));
    /// assert!(!NormalHair.is_valid(Hair::SnakeHair));
    ///
    /// assert!(!SnakeHair.is_valid(Hair::NoHair));
    /// assert!(!SnakeHair.is_valid(Hair::NormalHair{ style: HairStyle::Bun }));
    /// assert!(SnakeHair.is_valid(Hair::SnakeHair));
    /// ```
    pub fn is_valid(&self, hair: Hair) -> bool {
        match self {
            HairOption::NoHair => hair == Hair::NoHair,
            HairOption::NormalHair => matches!(hair, Hair::NormalHair { .. }),
            HairOption::SnakeHair => hair == Hair::SnakeHair,
        }
    }
}
