use crate::model::color::Color;

/// The hair of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hair {
    NoHair,
    NormalHair { color: HairColor, style: HairStyle },
    SnakeHair { color: Color, length: HairLength },
}

impl Hair {
    pub fn normal_hair(color: HairColor, style: HairStyle) -> Self {
        Self::NormalHair { color, style }
    }

    pub fn snake(color: Color, length: HairLength) -> Self {
        Self::SnakeHair { color, length }
    }
}

/// The hair color of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HairColor {
    Black,
    Brown,
    Red,
    Orange,
    Blond,
    Grey,
    White,
    Exotic(Color),
}

/// The hair style of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HairStyle {
    Bald,
    Shaved,
    Short(ShortHairStyle),
    Medium(MediumHairStyle),
    Long {
        style: LongHairStyle,
        length: HairLength,
    },
    Bun,
}

/// The short hair styles of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShortHairStyle {
    BuzzCut,
    CompOver,
    PixiCut,
    RegularHaircut,
}

/// The medium hair styles of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MediumHairStyle {
    Bob,
    Lob,
}

/// The long hair styles of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LongHairStyle {
    Braids,
    Curly,
    PigTails,
    Ponytail,
    Strait,
    Wavy,
}

/// The length of a [`long hair style`](LongHairStyle) of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HairLength {
    Shoulder,
    MidBack,
    Waist,
    Knee,
    Ankle,
}
