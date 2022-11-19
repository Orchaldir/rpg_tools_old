use crate::model::character::appearance::Appearance;
use crate::model::character::gender::Gender;
use crate::model::character::species::{Species, SpeciesId};
use anyhow::{bail, Result};

pub mod appearance;
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
    gender: Gender,
    appearance: Appearance,
}

impl Character {
    /// Creates a character, if valid.
    pub fn new<I: Into<CharacterId>>(
        id: I,
        species: &Species,
        gender: Gender,
        appearance: Appearance,
    ) -> Result<Self> {
        let id = id.into();

        if !species.gender_option().is_valid(gender) {
            bail!(
                "Character {} is invalid, because {:?} doesn't match the species's {:?}!",
                id.0,
                gender,
                species.gender_option()
            );
        } else if !species.appearance().is_valid(&appearance) {
            bail!(
                "Character {} is invalid, because {:?} doesn't match the species's {:?}!",
                id.0,
                appearance,
                species.appearance()
            );
        }

        Ok(Self {
            id,
            species: species.id(),
            gender,
            appearance,
        })
    }

    pub fn simple<I: Into<CharacterId>, S: Into<SpeciesId>>(
        id: I,
        species: S,
        gender: Gender,
    ) -> Self {
        Self {
            id: id.into(),
            species: species.into(),
            gender,
            appearance: Appearance::default(),
        }
    }

    pub fn id(&self) -> CharacterId {
        self.id
    }

    pub fn species(&self) -> SpeciesId {
        self.species
    }

    pub fn gender(&self) -> Gender {
        self.gender
    }

    pub fn appearance(&self) -> &Appearance {
        &self.appearance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::character::appearance::beard::Beard;
    use crate::model::character::appearance::hair::{Hair, HairColor, HairStyle};
    use crate::model::character::appearance::skin::Skin;
    use crate::model::character::gender::Gender;
    use crate::model::character::species::appearance::beard::BeardOptions;
    use crate::model::character::species::appearance::hair::HairOptions;
    use crate::model::character::species::appearance::skin::SkinOptions;
    use crate::model::character::species::appearance::AppearanceOptions;
    use crate::model::character::species::gender::GenderOption::TwoGenders;
    use Gender::*;

    #[test]
    fn test_validate_gender() {
        let appearance = Appearance::default();
        let species = Species::new(32, "test", TwoGenders, AppearanceOptions::default()).unwrap();

        assert!(Character::new(11, &species, Female, appearance).is_ok());
        assert!(Character::new(11, &species, Male, appearance).is_ok());
        assert!(Character::new(11, &species, Genderless, appearance).is_err());
    }

    #[test]
    fn test_validate_appearance() {
        let hair = HairOptions::new_hair([HairColor::Brown]).unwrap();
        let appearance =
            AppearanceOptions::new(BeardOptions::default(), hair, SkinOptions::default());
        let species = Species::new(32, "test", TwoGenders, appearance).unwrap();

        validate_appearance(&species, HairColor::Brown, true);
        validate_appearance(&species, HairColor::Red, false);
    }

    fn validate_appearance(species: &Species, hair_color: HairColor, result: bool) {
        let hair = Hair::normal_hair(hair_color, HairStyle::Bun);
        let appearance = Appearance::new(Beard::default(), hair, Skin::default());

        assert_eq!(
            Character::new(11, &species, Male, appearance).is_ok(),
            result
        );
    }
}
