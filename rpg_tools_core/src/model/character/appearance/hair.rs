/// The hair of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hair {
    NoHair,
    NormalHair { style: HairStyle },
    SnakeHair { length: HairLength },
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
