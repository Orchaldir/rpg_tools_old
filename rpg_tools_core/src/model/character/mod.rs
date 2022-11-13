mod manager;

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

/// A character of the story.
/// It can be a player character (PC) or non-player character (NPC).
/// It can be a main character or a side character.
#[derive(Clone, Debug, PartialEq)]
pub struct Character {
    id: CharacterId,
}

impl Character {
    pub fn new(id: CharacterId) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> CharacterId {
        self.id
    }
}
