/// The skin type of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SkinType {
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
