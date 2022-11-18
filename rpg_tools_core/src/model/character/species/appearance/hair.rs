use crate::model::character::appearance::hair::{Hair, HairColor};
use anyhow::{bail, Result};
use std::collections::HashSet;

/// The available options of [`Hair`] for a [`Species`](crate::model::character::species::Species).
#[derive(Clone, Debug, PartialEq)]
pub enum HairOption {
    NoHair,
    NormalHair {
        available_colors: HashSet<HairColor>,
    },
    SnakeHair,
}

impl HairOption {
    pub fn new_hair<const N: usize>(available_colors: [HairColor; N]) -> Result<Self> {
        if available_colors.is_empty() {
            bail!("HairOption::NormalHair needs at least 1 available color!")
        }

        Ok(Self::NormalHair {
            available_colors: available_colors.into(),
        })
    }

    /// Is the [`Hair`] valid for this option?
    pub fn is_valid(&self, hair: &Hair) -> bool {
        match self {
            HairOption::NoHair => *hair == Hair::NoHair,
            HairOption::NormalHair { available_colors } => match hair {
                Hair::NoHair => false,
                Hair::NormalHair { color, .. } => available_colors.contains(color),
                Hair::SnakeHair { .. } => false,
            },
            HairOption::SnakeHair => matches!(hair, Hair::SnakeHair { .. }),
        }
    }
}

impl Default for HairOption {
    fn default() -> Self {
        HairOption::NoHair
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::character::appearance::hair::{HairLength, HairStyle};
    use crate::model::color::Color;

    #[test]
    fn test_valid_hair() {
        let hair_option = HairOption::new_hair([HairColor::Brown]).unwrap();
        let hair = Hair::normal_hair(HairColor::Brown, HairStyle::Bun);
        let snake = Hair::snake(Color::Green, HairLength::Shoulder);

        assert!(HairOption::NoHair.is_valid(&Hair::NoHair));
        assert!(!HairOption::NoHair.is_valid(&hair));
        assert!(!HairOption::NoHair.is_valid(&snake));

        assert!(!hair_option.is_valid(&Hair::NoHair));
        assert!(hair_option.is_valid(&hair));
        assert!(!hair_option.is_valid(&snake));

        assert!(!HairOption::SnakeHair.is_valid(&Hair::NoHair));
        assert!(!HairOption::SnakeHair.is_valid(&hair));
        assert!(HairOption::SnakeHair.is_valid(&snake));
    }

    #[test]
    fn test_valid_hair_colors() {
        let blue = HairColor::Exotic(Color::Blue);
        let option = HairOption::new_hair([HairColor::Blond, blue]).unwrap();

        assert_skin_color(&option, HairColor::Black, false);
        assert_skin_color(&option, HairColor::Brown, false);
        assert_skin_color(&option, HairColor::Red, false);
        assert_skin_color(&option, HairColor::Orange, false);
        assert_skin_color(&option, HairColor::Blond, true);
        assert_skin_color(&option, HairColor::Grey, false);
        assert_skin_color(&option, HairColor::White, false);
        assert_skin_color(&option, blue, true);
    }

    fn assert_skin_color(option: &HairOption, color: HairColor, result: bool) {
        let hair = Hair::normal_hair(color, HairStyle::Bun);
        assert_eq!(option.is_valid(&hair), result)
    }
}
