use crate::model::character::appearance::Appearance;
use crate::model::character::species::appearance::hair::HairOption;
use crate::model::character::species::appearance::skin::SkinOption;

pub mod hair;
pub mod skin;

/// How can a [`character`](crate::model::character::Character) of this [`species`](crate::model::character::species::Species) look like?
#[derive(Clone, Debug, PartialEq)]
pub struct AppearanceOptions {
    hair: HairOption,
    skin: SkinOption,
}

impl AppearanceOptions {
    pub fn new(hair: HairOption, skin: SkinOption) -> Self {
        Self { hair, skin }
    }

    pub fn hair(&self) -> &HairOption {
        &self.hair
    }

    pub fn skin(&self) -> &SkinOption {
        &self.skin
    }

    /// Is the [`Appearance`] valid for this option?
    pub fn is_valid(&self, appearance: &Appearance) -> bool {
        self.hair.is_valid(appearance.hair()) && self.skin.is_valid(appearance.skin())
    }
}

impl Default for AppearanceOptions {
    fn default() -> Self {
        Self::new(HairOption::default(), SkinOption::default())
    }
}
