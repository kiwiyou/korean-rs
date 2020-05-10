use crate::{Syllable, Choseong, Jungseong, Jongseong};

pub enum CharacterType {
    Choseong(Choseong),
    Jungseong(Jungseong),
    Jongseong(Jongseong),
}

pub struct SyllableDecompose {
    choseong: Option<CharacterType>,
    jungseong: Option<CharacterType>,
    jongseong: Option<CharacterType>,
}

impl Iterator for SyllableDecompose {
    type Item = CharacterType;
    fn next(&mut self) -> Option<Self::Item> {
        self.choseong.take().or_else(|| self.jungseong.take()).or_else(|| self.jongseong.take())
    }
}

pub struct SyllableCharDecompose {
    choseong: Option<char>,
    jungseong: Option<char>,
    jongseong: Option<char>,
}

impl Iterator for SyllableCharDecompose {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.choseong.take().or_else(|| self.jungseong.take()).or_else(|| self.jongseong.take())
    }
}

pub struct SyllableU32Decompose {
    choseong: Option<u32>,
    jungseong: Option<u32>,
    jongseong: Option<u32>,
}

impl Iterator for SyllableU32Decompose {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.choseong.take().or_else(|| self.jungseong.take()).or_else(|| self.jongseong.take())
    }
}

trait SyllableExt {
    fn to_decompose(self) -> SyllableDecompose;

    fn to_decompose_char(self) -> SyllableCharDecompose;

    fn to_decompose_u32(self) -> SyllableU32Decompose;
}

impl SyllableExt for Syllable {
    fn to_decompose(self) -> SyllableDecompose {
        let (choseong, jungseong, jongseong) = self.into();
        SyllableDecompose {
            choseong: Some(CharacterType::Choseong(choseong)),
            jungseong: Some(CharacterType::Jungseong(jungseong)),
            jongseong: jongseong.map(CharacterType::Jongseong),
        }
    }

    fn to_decompose_char(self) -> SyllableCharDecompose {
        let (choseong, jungseong, jongseong) = self.into();
        SyllableCharDecompose {
            choseong: Some(choseong),
            jungseong: Some(jungseong),
            jongseong,
        }
    }

    fn to_decompose_u32(self) -> SyllableU32Decompose {
        let (choseong, jungseong, jongseong) = self.into();
        SyllableU32Decompose {
            choseong: Some(choseong),
            jungseong: Some(jungseong),
            jongseong,
        }
    }
}
