use crate::model::character::species::SpeciesId;

pub mod gender;
pub mod manager;
pub mod species;

/// The id of a [`Character`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CharacterId(usize);

impl CharacterId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

impl From<usize> for CharacterId {
    fn from(value: usize) -> Self {
        CharacterId::new(value)
    }
}

/// A character of the story.
/// It can be a player character (PC) or non-player character (NPC).
/// It can be a main character or a side character.
#[derive(Clone, Debug, PartialEq)]
pub struct Character {
    id: CharacterId,
    species: SpeciesId,
}

impl Character {
    pub fn new<I: Into<CharacterId>, S: Into<SpeciesId>>(id: I, species: S) -> Self {
        Self {
            id: id.into(),
            species: species.into(),
        }
    }

    pub fn get_id(&self) -> CharacterId {
        self.id
    }

    pub fn get_species(&self) -> SpeciesId {
        self.species
    }
}
