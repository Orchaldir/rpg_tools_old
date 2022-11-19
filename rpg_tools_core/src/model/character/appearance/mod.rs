use crate::model::character::appearance::beard::Beard;
use crate::model::character::appearance::hair::Hair;
use crate::model::character::appearance::skin::Skin;

pub mod beard;
pub mod hair;
pub mod skin;

/// How does a [`character`](crate::model::character::Character) look like?
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Appearance {
    beard: Beard,
    hair: Hair,
    skin: Skin,
}

impl Appearance {
    pub fn new(beard: Beard, hair: Hair, skin: Skin) -> Self {
        Self { beard, hair, skin }
    }

    pub fn beard(&self) -> &Beard {
        &self.beard
    }

    pub fn hair(&self) -> &Hair {
        &self.hair
    }

    pub fn skin(&self) -> &Skin {
        &self.skin
    }
}

impl Default for Appearance {
    fn default() -> Self {
        Self::new(Beard::default(), Hair::default(), Skin::default())
    }
}
