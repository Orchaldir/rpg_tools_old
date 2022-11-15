use crate::model::character::species::{Species, SpeciesId};
use anyhow::Result;

/// Stores all the [`Species`]s.
#[derive(Default, Debug)]
pub struct SpeciesMgr {
    species: Vec<Species>,
}

impl SpeciesMgr {
    /// Uses the function *f* to create a [`Species`] with the next [`SpeciesId`].
    pub fn create<F>(&mut self, f: F) -> Result<SpeciesId>
    where
        F: FnOnce(SpeciesId) -> Result<Species>,
    {
        let id = SpeciesId::new(self.species.len());
        self.species.push(f(id)?);
        Ok(id)
    }

    pub fn get_all(&self) -> &Vec<Species> {
        &self.species
    }

    pub fn get(&self, id: SpeciesId) -> Option<&Species> {
        self.species.get(id.id())
    }

    pub fn get_mut(&mut self, id: SpeciesId) -> Option<&mut Species> {
        self.species.get_mut(id.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::character::species::gender::GenderOption;
    use GenderOption::*;

    #[test]
    fn test_create() {
        let mut manager = SpeciesMgr::default();

        let id0 = manager
            .create(|id| Species::new(id, "t0", NoGender))
            .unwrap();
        let id1 = manager
            .create(|id| Species::new(id, "t1", TwoGenders))
            .unwrap();

        assert_ne!(id0, id1);
        assert_eq!(id0, manager.get(id0).unwrap().get_id());
        assert_eq!(id1, manager.get(id1).unwrap().get_id());
    }
}
