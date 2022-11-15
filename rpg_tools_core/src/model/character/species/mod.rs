use crate::model::name::Name;
use anyhow::{Context, Result};

pub mod gender;

/// The id of a [`Species`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SpeciesId(usize);

impl SpeciesId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

impl From<usize> for SpeciesId {
    fn from(value: usize) -> Self {
        SpeciesId::new(value)
    }
}

/// The species of a [`character`](crate::model::character::Character).
#[derive(Clone, Debug, PartialEq)]
pub struct Species {
    id: SpeciesId,
    name: Name,
}

impl Species {
    pub fn new<I: Into<SpeciesId>, S: Into<String>>(id: I, name: S) -> Result<Self> {
        let id = id.into();
        let name = name.into();
        let name = Name::new(name).with_context(|| format!("Failed to create species {}", id.0))?;

        Ok(Self { id, name })
    }

    pub fn get_id(&self) -> SpeciesId {
        self.id
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(Species::new(0, "Test").is_ok());
        assert!(Species::new(SpeciesId::new(2), "Test2").is_ok());
    }
}
