/// The skin of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Skin {
    Skin(SkinColor),
    Scales,
}

/// The skin color of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SkinColor {
    Light,
    Medium,
    Warm,
    Tan,
    Dark,
    VeryDark,
}
