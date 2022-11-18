use crate::model::character::appearance::Appearance;
use crate::model::character::species::appearance::beard::BeardOption;
use crate::model::character::species::appearance::hair::HairOption;
use crate::model::character::species::appearance::skin::SkinOption;

pub mod beard;
pub mod hair;
pub mod skin;

/// How can a [`character`](crate::model::character::Character) of this [`species`](crate::model::character::species::Species) look like?
#[derive(Clone, Debug, PartialEq)]
pub struct AppearanceOptions {
    beard: BeardOption,
    hair: HairOption,
    skin: SkinOption,
}

impl AppearanceOptions {
    pub fn new(beard: BeardOption, hair: HairOption, skin: SkinOption) -> Self {
        Self { beard, hair, skin }
    }

    pub fn beard(&self) -> &BeardOption {
        &self.beard
    }

    pub fn hair(&self) -> &HairOption {
        &self.hair
    }

    pub fn skin(&self) -> &SkinOption {
        &self.skin
    }

    /// Is the [`Appearance`] valid for this option?
    pub fn is_valid(&self, appearance: &Appearance) -> bool {
        self.beard.is_valid(appearance.beard())
            && self.hair.is_valid(appearance.hair())
            && self.skin.is_valid(appearance.skin())
    }
}

impl Default for AppearanceOptions {
    fn default() -> Self {
        Self::new(
            BeardOption::default(),
            HairOption::default(),
            SkinOption::default(),
        )
    }
}
