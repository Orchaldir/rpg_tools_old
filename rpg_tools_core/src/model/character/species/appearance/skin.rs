use crate::model::character::appearance::skin::{Skin, SkinColor};
use crate::model::color::Color;
use anyhow::{bail, Result};
use std::collections::HashSet;

/// The available options of [`Skin`] for a [`Species`](crate::model::character::species::Species).
#[derive(Clone, Debug, PartialEq)]
pub enum SkinOption {
    HasSkin {
        available_colors: HashSet<SkinColor>,
    },
    HasScales,
}

impl SkinOption {
    pub fn new_skin<const N: usize>(available_colors: [SkinColor; N]) -> Result<Self> {
        if available_colors.is_empty() {
            bail!("SkinOption::HasSkin needs at least 1 available color!")
        }

        Ok(Self::HasSkin {
            available_colors: available_colors.into(),
        })
    }

    /// Is the [`Skin`] valid for this option?
    pub fn is_valid(&self, skin: Skin) -> bool {
        match self {
            SkinOption::HasSkin { available_colors } => match skin {
                Skin::Skin(color) => available_colors.contains(&color),
                Skin::Scales => false,
            },
            SkinOption::HasScales => skin == Skin::Scales,
        }
    }
}

impl Default for SkinOption {
    fn default() -> Self {
        SkinOption::new_skin([SkinColor::Exotic(Color::Aqua)]).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_skin() {
        let skin_option = SkinOption::new_skin([SkinColor::Light]).unwrap();
        let skin = Skin::Skin(SkinColor::Medium);

        assert!(!skin_option.is_valid(Skin::Scales));
        assert!(!SkinOption::HasScales.is_valid(skin));
        assert!(SkinOption::HasScales.is_valid(Skin::Scales));
    }

    #[test]
    fn test_valid_skin_colors() {
        let option = SkinOption::new_skin([SkinColor::Warm, SkinColor::Tan]).unwrap();

        assert_skin_color(&option, SkinColor::Light, false);
        assert_skin_color(&option, SkinColor::Medium, false);
        assert_skin_color(&option, SkinColor::Warm, true);
        assert_skin_color(&option, SkinColor::Tan, true);
        assert_skin_color(&option, SkinColor::Dark, false);
        assert_skin_color(&option, SkinColor::VeryDark, false);
    }

    fn assert_skin_color(option: &SkinOption, color: SkinColor, result: bool) {
        assert_eq!(option.is_valid(Skin::Skin(color)), result)
    }
}
