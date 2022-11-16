use crate::model::character::appearance::hair::Hair;

pub mod hair;

/// How does a [`character`](crate::model::character::Character) look like?
#[derive(Clone, Debug, PartialEq)]
pub struct Appearance {
    hair: Hair,
}

impl Appearance {
    pub fn new(hair: Hair) -> Self {
        Self { hair }
    }

    pub fn get_hair(&self) -> Hair {
        self.hair
    }
}
