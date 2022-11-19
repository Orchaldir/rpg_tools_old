use crate::model::character::appearance::hair::{Hair, HairColor};
use anyhow::{bail, Result};
use std::collections::HashSet;

/// The available options of [`Hair`] for a [`Species`](crate::model::character::species::Species).
#[derive(Clone, Debug, PartialEq)]
pub enum HairOptions {
    NoHair,
    NormalHair {
        available_colors: HashSet<HairColor>,
    },
    SnakeHair,
}

impl HairOptions {
    pub fn new_hair<const N: usize>(available_colors: [HairColor; N]) -> Result<Self> {
        if available_colors.is_empty() {
            bail!("HairOptions::NormalHair needs at least 1 available color!")
        }

        Ok(Self::NormalHair {
            available_colors: available_colors.into(),
        })
    }

    /// Is the [`Hair`] valid for this option?
    pub fn is_valid(&self, hair: &Hair) -> bool {
        match self {
            HairOptions::NoHair => *hair == Hair::NoHair,
            HairOptions::NormalHair { available_colors } => match hair {
                Hair::NoHair => false,
                Hair::NormalHair { color, .. } => available_colors.contains(color),
                Hair::SnakeHair { .. } => false,
            },
            HairOptions::SnakeHair => matches!(hair, Hair::SnakeHair { .. }),
        }
    }
}

impl Default for HairOptions {
    fn default() -> Self {
        HairOptions::NoHair
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::character::appearance::hair::{HairLength, HairStyle};
    use crate::model::color::Color;

    #[test]
    fn test_valid_hair() {
        let hair_option = HairOptions::new_hair([HairColor::Brown]).unwrap();
        let hair = Hair::normal_hair(HairColor::Brown, HairStyle::Bun);
        let snake = Hair::snake(Color::Green, HairLength::Shoulder);

        assert!(HairOptions::NoHair.is_valid(&Hair::NoHair));
        assert!(!HairOptions::NoHair.is_valid(&hair));
        assert!(!HairOptions::NoHair.is_valid(&snake));

        assert!(!hair_option.is_valid(&Hair::NoHair));
        assert!(hair_option.is_valid(&hair));
        assert!(!hair_option.is_valid(&snake));

        assert!(!HairOptions::SnakeHair.is_valid(&Hair::NoHair));
        assert!(!HairOptions::SnakeHair.is_valid(&hair));
        assert!(HairOptions::SnakeHair.is_valid(&snake));
    }

    #[test]
    fn test_valid_hair_colors() {
        let blue = HairColor::Exotic(Color::Blue);
        let option = HairOptions::new_hair([HairColor::Blond, blue]).unwrap();

        assert_skin_color(&option, HairColor::Black, false);
        assert_skin_color(&option, HairColor::Brown, false);
        assert_skin_color(&option, HairColor::Red, false);
        assert_skin_color(&option, HairColor::Orange, false);
        assert_skin_color(&option, HairColor::Blond, true);
        assert_skin_color(&option, HairColor::Grey, false);
        assert_skin_color(&option, HairColor::White, false);
        assert_skin_color(&option, blue, true);
    }

    fn assert_skin_color(option: &HairOptions, color: HairColor, result: bool) {
        let hair = Hair::normal_hair(color, HairStyle::Bun);
        assert_eq!(option.is_valid(&hair), result)
    }
}
