use crate::model::character::species::appearance::hair::HairOption;

pub mod hair;

/// How can a [`character`](crate::model::character::Character) of this [`species`](crate::model::character::species::Species) look like?
#[derive(Clone, Debug, PartialEq)]
pub struct AppearanceOptions {
    hair: HairOption,
}

impl AppearanceOptions {
    pub fn new(hair: HairOption) -> Self {
        Self { hair }
    }

    pub fn get_hair(&self) -> HairOption {
        self.hair
    }
}
