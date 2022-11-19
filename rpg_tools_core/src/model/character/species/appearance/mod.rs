use crate::model::character::appearance::Appearance;
use crate::model::character::species::appearance::beard::BeardOptions;
use crate::model::character::species::appearance::hair::HairOptions;
use crate::model::character::species::appearance::skin::SkinOptions;

pub mod beard;
pub mod hair;
pub mod skin;

/// How can a [`character`](crate::model::character::Character) of this [`species`](crate::model::character::species::Species) look like?
#[derive(Clone, Debug, PartialEq)]
pub struct AppearanceOptions {
    beard: BeardOptions,
    hair: HairOptions,
    skin: SkinOptions,
}

impl AppearanceOptions {
    pub fn new(beard: BeardOptions, hair: HairOptions, skin: SkinOptions) -> Self {
        Self { beard, hair, skin }
    }

    pub fn beard(&self) -> &BeardOptions {
        &self.beard
    }

    pub fn hair(&self) -> &HairOptions {
        &self.hair
    }

    pub fn skin(&self) -> &SkinOptions {
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
            BeardOptions::default(),
            HairOptions::default(),
            SkinOptions::default(),
        )
    }
}
