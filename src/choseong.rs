use crate::characters::{ChoseongCharacter::*, *};
use crate::constants::*;
use crate::syllable::*;
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Choseong {
    Normal(ChoseongCharacter),
    Compat(ChoseongCharacter),
}

fn to_code(code: u32) -> u32 {
    if code.is_syllable() {
        let value = (code - HANGEUL_OFFSET) / (JUNGSEONG_COUNT * JONGSEONG_COUNT);
        value + CHOSEONG_START
    } else {
        code
    }
}

pub trait ChoseongInformation {
    fn is_choseong(&self) -> bool;
    fn is_normal_choseong(&self) -> bool;
    fn is_compat_choseong(&self) -> bool;
    fn has_choseong(&self) -> bool;
    fn has_normal_choseong(&self) -> bool;
    fn has_compat_choseong(&self) -> bool;
}

impl ChoseongInformation for u32 {
    fn is_choseong(&self) -> bool {
        Choseong::try_from(*self).is_ok()
    }

    fn is_normal_choseong(&self) -> bool {
        match Choseong::try_from(*self) {
            Ok(choseong) => match choseong {
                Choseong::Normal(_) => true,
                Choseong::Compat(_) => false,
            },
            Err(_) => false,
        }
    }

    fn is_compat_choseong(&self) -> bool {
        match Choseong::try_from(*self) {
            Ok(choseong) => match choseong {
                Choseong::Normal(_) => false,
                Choseong::Compat(_) => true,
            },
            Err(_) => false,
        }
    }

    fn has_choseong(&self) -> bool {
        to_code(*self).is_choseong()
    }

    fn has_normal_choseong(&self) -> bool {
        to_code(*self).is_normal_choseong()
    }

    fn has_compat_choseong(&self) -> bool {
        to_code(*self).is_compat_choseong()
    }
}

impl ChoseongInformation for char {
    fn is_choseong(&self) -> bool {
        (*self as u32).is_choseong()
    }

    fn is_normal_choseong(&self) -> bool {
        (*self as u32).is_normal_choseong()
    }

    fn is_compat_choseong(&self) -> bool {
        (*self as u32).is_compat_choseong()
    }

    fn has_choseong(&self) -> bool {
        (*self as u32).has_choseong()
    }

    fn has_normal_choseong(&self) -> bool {
        (*self as u32).has_normal_choseong()
    }

    fn has_compat_choseong(&self) -> bool {
        (*self as u32).has_compat_choseong()
    }
}

impl From<&Choseong> for ChoseongCharacter {
    fn from(item: &Choseong) -> ChoseongCharacter {
        match item {
            Choseong::Normal(character) => character.clone(),
            Choseong::Compat(character) => character.clone(),
        }
    }
}

impl ChoseongCharacter {
    pub fn to_normal(&self) -> Choseong {
        Choseong::Normal(self.clone())
    }

    pub fn to_compat(&self) -> Choseong {
        Choseong::Compat(self.clone())
    }
}

impl From<&ChoseongCharacter> for Choseong {
    fn from(item: &ChoseongCharacter) -> Choseong {
        Choseong::Normal(item.clone())
    }
}

impl TryFrom<u32> for Choseong {
    type Error = ();

    fn try_from(item: u32) -> Result<Self, Self::Error> {
        let character = match to_code(item) {
            0x1100 => Choseong::Normal(Giyeok),
            0x1101 => Choseong::Normal(SsangGiyeok),
            0x1102 => Choseong::Normal(Nieun),
            0x1103 => Choseong::Normal(Digeut),
            0x1104 => Choseong::Normal(SsangDigeut),
            0x1105 => Choseong::Normal(Rieul),
            0x1106 => Choseong::Normal(Mieum),
            0x1107 => Choseong::Normal(Bieup),
            0x1108 => Choseong::Normal(SsangBieup),
            0x1109 => Choseong::Normal(Siot),
            0x110A => Choseong::Normal(SsangSiot),
            0x110B => Choseong::Normal(Ieung),
            0x110C => Choseong::Normal(Jieut),
            0x110D => Choseong::Normal(SsangJieut),
            0x110E => Choseong::Normal(Chieut),
            0x110F => Choseong::Normal(Kiyeok),
            0x1110 => Choseong::Normal(Tieut),
            0x1111 => Choseong::Normal(Pieup),
            0x1112 => Choseong::Normal(Hieuh),

            0x3131 => Choseong::Compat(Giyeok),
            0x3132 => Choseong::Compat(SsangGiyeok),
            0x3134 => Choseong::Compat(Nieun),
            0x3137 => Choseong::Compat(Digeut),
            0x3138 => Choseong::Compat(SsangDigeut),
            0x3139 => Choseong::Compat(Rieul),
            0x3141 => Choseong::Compat(Mieum),
            0x3142 => Choseong::Compat(Bieup),
            0x3143 => Choseong::Compat(SsangBieup),
            0x3145 => Choseong::Compat(Siot),
            0x3146 => Choseong::Compat(SsangSiot),
            0x3147 => Choseong::Compat(Ieung),
            0x3148 => Choseong::Compat(Jieut),
            0x3149 => Choseong::Compat(SsangJieut),
            0x313A => Choseong::Compat(Chieut),
            0x314B => Choseong::Compat(Kiyeok),
            0x314C => Choseong::Compat(Tieut),
            0x314D => Choseong::Compat(Pieup),
            0x314E => Choseong::Compat(Hieuh),
            _ => return Err(()),
        };

        Ok(character)
    }
}

impl From<&Choseong> for u32 {
    fn from(item: &Choseong) -> Self {
        match item {
            Choseong::Normal(character) => match character {
                Giyeok => 0x1100,
                SsangGiyeok => 0x1101,
                Nieun => 0x1102,
                Digeut => 0x1103,
                SsangDigeut => 0x1104,
                Rieul => 0x1105,
                Mieum => 0x1106,
                Bieup => 0x1107,
                SsangBieup => 0x1108,
                Siot => 0x1109,
                SsangSiot => 0x110A,
                Ieung => 0x110B,
                Jieut => 0x110C,
                SsangJieut => 0x110D,
                Chieut => 0x110E,
                Kiyeok => 0x110F,
                Tieut => 0x1110,
                Pieup => 0x1111,
                Hieuh => 0x1112,
            },
            Choseong::Compat(character) => match character {
                Giyeok => 0x3131,
                SsangGiyeok => 0x3132,
                Nieun => 0x3134,
                Digeut => 0x3137,
                SsangDigeut => 0x3138,
                Rieul => 0x3139,
                Mieum => 0x3141,
                Bieup => 0x3142,
                SsangBieup => 0x3143,
                Siot => 0x3145,
                SsangSiot => 0x3146,
                Ieung => 0x3147,
                Jieut => 0x3148,
                SsangJieut => 0x3149,
                Chieut => 0x313A,
                Kiyeok => 0x314B,
                Tieut => 0x314C,
                Pieup => 0x314D,
                Hieuh => 0x314E,
            },
        }
    }
}

impl TryFrom<char> for Choseong {
    type Error = ();

    fn try_from(item: char) -> Result<Self, Self::Error> {
        Choseong::try_from(item as u32)
    }
}

impl From<&Choseong> for char {
    fn from(item: &Choseong) -> char {
        match item {
            Choseong::Normal(character) => match character {
                Giyeok => '\u{1100}',
                SsangGiyeok => '\u{1101}',
                Nieun => '\u{1102}',
                Digeut => '\u{1103}',
                SsangDigeut => '\u{1104}',
                Rieul => '\u{1105}',
                Mieum => '\u{1106}',
                Bieup => '\u{1107}',
                SsangBieup => '\u{1108}',
                Siot => '\u{1109}',
                SsangSiot => '\u{110A}',
                Ieung => '\u{110B}',
                Jieut => '\u{110C}',
                SsangJieut => '\u{110D}',
                Chieut => '\u{110E}',
                Kiyeok => '\u{110F}',
                Tieut => '\u{1110}',
                Pieup => '\u{1111}',
                Hieuh => '\u{1112}',
            },
            Choseong::Compat(character) => match character {
                Giyeok => '\u{3131}',
                SsangGiyeok => '\u{3132}',
                Nieun => '\u{3134}',
                Digeut => '\u{3137}',
                SsangDigeut => '\u{3138}',
                Rieul => '\u{3139}',
                Mieum => '\u{3141}',
                Bieup => '\u{3142}',
                SsangBieup => '\u{3143}',
                Siot => '\u{3145}',
                SsangSiot => '\u{3146}',
                Ieung => '\u{3147}',
                Jieut => '\u{3148}',
                SsangJieut => '\u{3149}',
                Chieut => '\u{313A}',
                Kiyeok => '\u{314B}',
                Tieut => '\u{314C}',
                Pieup => '\u{314D}',
                Hieuh => '\u{314E}',
            },
        }
    }
}

impl CharacterInformation for Choseong {
    fn is_jaeum(&self) -> bool {
        match self {
            Choseong::Normal(character) => character.is_jaeum(),
            Choseong::Compat(character) => character.is_jaeum(),
        }
    }

    fn is_moeum(&self) -> bool {
        match self {
            Choseong::Normal(character) => character.is_moeum(),
            Choseong::Compat(character) => character.is_moeum(),
        }
    }

    fn to_composable(&self) -> u32 {
        match self {
            Choseong::Normal(character) => character.to_composable(),
            Choseong::Compat(character) => character.to_composable(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NON_NORMAL_CHOSEONG_START_U32: u32 = 0x1099;
    const NON_NORMAL_CHOSEONG_END_U32: u32 = 0x1113;
    const NORMAL_CHOSEONG_U32_LIST: [u32; 19] = [
        0x1100, 0x1101, 0x1102, 0x1103, 0x1104, 0x1105, 0x1106, 0x1107, 0x1108, 0x1109, 0x110A,
        0x110B, 0x110C, 0x110D, 0x110E, 0x110F, 0x1110, 0x1111, 0x1112,
    ];

    const NON_COMPAT_CHOSEONG_START_U32: u32 = 0x3130;
    const NON_COMPAT_CHOSEONG_END_U32: u32 = 0x314F;
    const COMPAT_CHOSEONG_U32_LIST: [u32; 19] = [
        0x3131, 0x3132, 0x3134, 0x3137, 0x3138, 0x3139, 0x3141, 0x3142, 0x3143, 0x3145, 0x3146,
        0x3147, 0x3148, 0x3149, 0x313A, 0x314B, 0x314C, 0x314D, 0x314E,
    ];

    const NON_NORMAL_CHOSEONG_START_CHAR: char = '\u{1099}';
    const NON_NORMAL_CHOSEONG_END_CHAR: char = '\u{1113}';
    const NORMAL_CHOSEONG_CHAR_LIST: [char; 19] = [
        'ᄀ', 'ᄁ', 'ᄂ', 'ᄃ', 'ᄄ', 'ᄅ', 'ᄆ', 'ᄇ', 'ᄈ', 'ᄉ', 'ᄊ', 'ᄋ', 'ᄌ', 'ᄍ', 'ᄎ',
        'ᄏ', 'ᄐ', 'ᄑ', 'ᄒ',
    ];

    const NON_COMPAT_CHOSEONG_START_CHAR: char = '\u{3130}';
    const NON_COMPAT_CHOSEONG_END_CHAR: char = '\u{314F}';
    const COMPAT_CHOSEONG_CHAR_LIST: [char; 19] = [
        'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㄺ',
        'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
    ];

    #[test]
    fn is_choseong_with_u32() {
        assert_eq!(NON_NORMAL_CHOSEONG_START_U32.is_choseong(), false);
        for choseong in NORMAL_CHOSEONG_U32_LIST.iter() {
            assert_eq!(choseong.is_choseong(), true);
        }
        assert_eq!(NON_NORMAL_CHOSEONG_END_U32.is_choseong(), false);

        assert_eq!(NON_COMPAT_CHOSEONG_START_U32.is_choseong(), false);
        for choseong in COMPAT_CHOSEONG_U32_LIST.iter() {
            assert_eq!(choseong.is_choseong(), true);
        }
        assert_eq!(NON_COMPAT_CHOSEONG_END_U32.is_choseong(), false);
    }

    #[test]
    fn is_choseong_with_char() {
        assert_eq!(NON_NORMAL_CHOSEONG_START_CHAR.is_choseong(), false);
        for choseong in NORMAL_CHOSEONG_CHAR_LIST.iter() {
            assert_eq!(choseong.is_choseong(), true);
        }
        assert_eq!(NON_NORMAL_CHOSEONG_END_CHAR.is_choseong(), false);

        assert_eq!(NON_COMPAT_CHOSEONG_START_CHAR.is_choseong(), false);
        for choseong in COMPAT_CHOSEONG_CHAR_LIST.iter() {
            assert_eq!(choseong.is_choseong(), true);
        }
        assert_eq!(NON_COMPAT_CHOSEONG_END_CHAR.is_choseong(), false);
    }

    #[test]
    fn is_normal_choseong_with_u32() {
        assert_eq!(NON_NORMAL_CHOSEONG_START_U32.is_normal_choseong(), false);
        for choseong in NORMAL_CHOSEONG_U32_LIST.iter() {
            assert_eq!(choseong.is_normal_choseong(), true);
        }
        assert_eq!(NON_NORMAL_CHOSEONG_END_U32.is_normal_choseong(), false);

        assert_eq!(NON_COMPAT_CHOSEONG_START_U32.is_normal_choseong(), false);
        for choseong in COMPAT_CHOSEONG_U32_LIST.iter() {
            assert_eq!(choseong.is_normal_choseong(), false);
        }
        assert_eq!(NON_COMPAT_CHOSEONG_END_U32.is_normal_choseong(), false);
    }

    #[test]
    fn is_normal_choseong_with_char() {
        assert_eq!(NON_NORMAL_CHOSEONG_START_CHAR.is_normal_choseong(), false);
        for choseong in NORMAL_CHOSEONG_CHAR_LIST.iter() {
            assert_eq!(choseong.is_normal_choseong(), true);
        }
        assert_eq!(NON_NORMAL_CHOSEONG_END_CHAR.is_normal_choseong(), false);

        assert_eq!(NON_COMPAT_CHOSEONG_START_CHAR.is_normal_choseong(), false);
        for choseong in COMPAT_CHOSEONG_CHAR_LIST.iter() {
            assert_eq!(choseong.is_normal_choseong(), false);
        }
        assert_eq!(NON_COMPAT_CHOSEONG_END_CHAR.is_normal_choseong(), false);
    }

    #[test]
    fn is_compat_choseong_with_u32() {
        assert_eq!(NON_NORMAL_CHOSEONG_START_U32.is_compat_choseong(), false);
        for choseong in NORMAL_CHOSEONG_U32_LIST.iter() {
            assert_eq!(choseong.is_compat_choseong(), false);
        }
        assert_eq!(NON_NORMAL_CHOSEONG_END_U32.is_compat_choseong(), false);

        assert_eq!(NON_COMPAT_CHOSEONG_START_U32.is_compat_choseong(), false);
        for choseong in COMPAT_CHOSEONG_U32_LIST.iter() {
            assert_eq!(choseong.is_compat_choseong(), true);
        }
        assert_eq!(NON_COMPAT_CHOSEONG_END_U32.is_compat_choseong(), false);
    }

    #[test]
    fn is_compat_choseong_with_char() {
        assert_eq!(NON_NORMAL_CHOSEONG_START_CHAR.is_compat_choseong(), false);
        for choseong in NORMAL_CHOSEONG_CHAR_LIST.iter() {
            assert_eq!(choseong.is_compat_choseong(), false);
        }
        assert_eq!(NON_NORMAL_CHOSEONG_END_CHAR.is_compat_choseong(), false);

        assert_eq!(NON_COMPAT_CHOSEONG_START_CHAR.is_compat_choseong(), false);
        for choseong in COMPAT_CHOSEONG_CHAR_LIST.iter() {
            assert_eq!(choseong.is_compat_choseong(), true);
        }
        assert_eq!(NON_COMPAT_CHOSEONG_END_CHAR.is_compat_choseong(), false);
    }
}
