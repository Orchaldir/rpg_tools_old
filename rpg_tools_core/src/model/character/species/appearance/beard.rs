use crate::model::character::appearance::beard::Beard;
use crate::model::character::appearance::hair::HairColor;
use anyhow::{bail, Result};
use std::collections::HashSet;

/// The available options of [`Beard`] for a [`Species`](crate::model::character::species::Species).
#[derive(Clone, Debug, PartialEq)]
pub enum BeardOption {
    NoBeard,
    Beard {
        available_colors: HashSet<HairColor>,
    },
}

impl BeardOption {
    pub fn new_beard<const N: usize>(available_colors: [HairColor; N]) -> Result<Self> {
        if available_colors.is_empty() {
            bail!("BeardOption::Beard needs at least 1 available color!")
        }

        Ok(Self::Beard {
            available_colors: available_colors.into(),
        })
    }

    /// Is the [`Beard`] valid for this option?
    pub fn is_valid(&self, beard: &Beard) -> bool {
        match self {
            BeardOption::NoBeard => *beard == Beard::NoBeard,
            BeardOption::Beard { available_colors } => match beard {
                Beard::NoBeard => false,
                Beard::Beard { color, .. } => available_colors.contains(color),
            },
        }
    }
}

impl Default for BeardOption {
    fn default() -> Self {
        BeardOption::NoBeard
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::character::appearance::beard::BeardStyle;
    use crate::model::color::Color;

    #[test]
    fn test_valid_hair() {
        let beard_option = BeardOption::new_beard([HairColor::Brown]).unwrap();
        let beard = Beard::new_beard(HairColor::Brown, BeardStyle::Goatee);

        assert!(BeardOption::NoBeard.is_valid(&Beard::NoBeard));
        assert!(!BeardOption::NoBeard.is_valid(&beard));

        assert!(!beard_option.is_valid(&Beard::NoBeard));
        assert!(beard_option.is_valid(&beard));
    }

    #[test]
    fn test_valid_hair_colors() {
        let blue = HairColor::Exotic(Color::Blue);
        let option = BeardOption::new_beard([HairColor::Blond, blue]).unwrap();

        assert_skin_color(&option, HairColor::Black, false);
        assert_skin_color(&option, HairColor::Brown, false);
        assert_skin_color(&option, HairColor::Red, false);
        assert_skin_color(&option, HairColor::Orange, false);
        assert_skin_color(&option, HairColor::Blond, true);
        assert_skin_color(&option, HairColor::Grey, false);
        assert_skin_color(&option, HairColor::White, false);
        assert_skin_color(&option, blue, true);
    }

    fn assert_skin_color(option: &BeardOption, color: HairColor, result: bool) {
        let beard = Beard::new_beard(color, BeardStyle::Imperial);
        assert_eq!(option.is_valid(&beard), result)
    }
}
