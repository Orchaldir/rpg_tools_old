use crate::model::character::appearance::hair::Hair;
use crate::model::character::appearance::skin::Skin;

pub mod hair;
pub mod skin;

/// How does a [`character`](crate::model::character::Character) look like?
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Appearance {
    hair: Hair,
    skin: Skin,
}

impl Appearance {
    pub fn new(hair: Hair, skin: Skin) -> Self {
        Self { hair, skin }
    }

    pub fn get_hair(&self) -> Hair {
        self.hair
    }

    pub fn get_skin(&self) -> Skin {
        self.skin
    }
}

impl Default for Appearance {
    fn default() -> Self {
        Self::new(Hair::default(), Skin::default())
    }
}
