// use regex::Regex;
use std::char;

pub const CONSONANTS: [char; 30] = [
    'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ',
    'ㄾ', 'ㄿ', 'ㅀ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ',
    'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
];
pub const VOWELS: [char; 21] = [
    'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ',
    'ㅜ', 'ㅝ', 'ㅞ', 'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
];

pub const CHOSEONGS: [char; 19] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ',
    'ㅉ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
];
pub const JUNGSEONGS: [char; 21] = VOWELS;
pub const JONGSEONGS: [char; 28] = [
    '\0', 'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ',
    'ㄾ', 'ㄿ', 'ㅀ', 'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ',
    'ㅍ', 'ㅎ',
];

pub const COMPLEX_CONSONANTS: [(char, [char; 2]); 10] = [
    ('ㄳ', ['ㄱ', 'ㅅ']),
    ('ㄵ', ['ㄴ', 'ㅈ']),
    ('ㄺ', ['ㄹ', 'ㄱ']),
    ('ㄻ', ['ㄹ', 'ㅁ']),
    ('ㄼ', ['ㄹ', 'ㅂ']),
    ('ㄽ', ['ㄹ', 'ㅅ']),
    ('ㄾ', ['ㄹ', 'ㅌ']),
    ('ㄿ', ['ㄹ', 'ㅍ']),
    ('ㅀ', ['ㄹ', 'ㅎ']),
    ('ㅄ', ['ㅂ', 'ㅅ']),
];
pub const COMPLEX_VOWELS: [(char, [char; 2]); 7] = [
    ('ㅘ', ['ㅗ', 'ㅏ']),
    ('ㅙ', ['ㅗ', 'ㅐ']),
    ('ㅚ', ['ㅗ', 'ㅣ']),
    ('ㅝ', ['ㅜ', 'ㅓ']),
    ('ㅞ', ['ㅜ', 'ㅔ']),
    ('ㅟ', ['ㅜ', 'ㅣ']),
    ('ㅢ', ['ㅡ', 'ㅣ']),
];

// lazy_static! {
//     static ref HANGUL: Regex = Regex::new("^[가-힣ㄱ-ㅎㅏ-ㅣ]+$").unwrap();
//     static ref HANGUL_COMPELETE: Regex = Regex::new("^[가-힣]+$").unwrap();
// }

pub fn assemble(phoneme_codes: Vec<u32>) -> Result<u32, ()> {
    match phoneme_codes.len() {
        // ETC
        1 => Ok(phoneme_codes[0]),
        // Consonant + Vowel
        2 => match (
            consonant_to_choseong_index(phoneme_codes[0]),
            vowel_to_jungseong_index(phoneme_codes[1]),
        ) {
            (Some(cho), Some(jung)) => Ok((44032 + (cho * 588) + (jung * 28)) as u32),
            _ => Err(()),
        },
        // (Consonant + Vowel + Consonant) or (Consonant + Vowel + Vowel)
        3 => {
            if is_consonant(phoneme_codes[0])
                && is_vowel(phoneme_codes[1])
                && is_consonant(phoneme_codes[2])
            {
                let cho = consonant_to_choseong_index(phoneme_codes[0]).unwrap();
                let jung = vowel_to_jungseong_index(phoneme_codes[1]).unwrap();
                let jong = consonant_to_jongseong_index(phoneme_codes[2]).unwrap();
                Ok((44032 + (cho * 588) + (jung * 28) + jong) as u32)
            } else if is_consonant(phoneme_codes[0])
                && is_simple_vowel(phoneme_codes[1])
                && is_simple_vowel(phoneme_codes[2])
            {
                let cho = consonant_to_choseong_index(phoneme_codes[0]).unwrap();
                let jung = vowel_to_jungseong_index(
                    assemble_vowel((phoneme_codes[1], phoneme_codes[2])).unwrap(),
                )
                .unwrap();
                Ok((44032 + (cho * 588) + (jung * 28)) as u32)
            } else {
                Err(())
            }
        }
        // (Consonant + Vowel + Consonant + Consonant) or (Consonant + Vowel + Vowel + Consonant)
        4 => {
            if is_consonant(phoneme_codes[0])
                && is_vowel(phoneme_codes[1])
                && is_simple_consonant(phoneme_codes[2])
                && is_simple_consonant(phoneme_codes[3])
            {
                let cho = consonant_to_choseong_index(phoneme_codes[0]).unwrap();
                let jung = vowel_to_jungseong_index(phoneme_codes[1]).unwrap();
                let jong = consonant_to_jongseong_index(
                    assemble_consonant((phoneme_codes[2], phoneme_codes[3])).unwrap(),
                )
                .unwrap();
                Ok((44032 + (cho * 588) + (jung * 28) + jong) as u32)
            } else if is_consonant(phoneme_codes[0])
                && is_simple_vowel(phoneme_codes[1])
                && is_simple_vowel(phoneme_codes[2])
                && is_consonant(phoneme_codes[3])
            {
                let cho = consonant_to_choseong_index(phoneme_codes[0]).unwrap();
                let jung = vowel_to_jungseong_index(
                    assemble_vowel((phoneme_codes[1], phoneme_codes[2])).unwrap(),
                )
                .unwrap();
                let jong = consonant_to_jongseong_index(phoneme_codes[3]).unwrap();
                Ok((44032 + (cho * 588) + (jung * 28) + jong) as u32)
            } else {
                Err(())
            }
        }
        // Consonant + Vowel + Vowel + Consonant + Consonant
        5 => {
            if is_consonant(phoneme_codes[0])
                && is_simple_vowel(phoneme_codes[1])
                && is_simple_vowel(phoneme_codes[2])
                && is_simple_consonant(phoneme_codes[3])
                && is_simple_consonant(phoneme_codes[4])
            {
                let cho = consonant_to_choseong_index(phoneme_codes[0]).unwrap();
                let jung = vowel_to_jungseong_index(
                    assemble_vowel((phoneme_codes[1], phoneme_codes[2])).unwrap(),
                )
                .unwrap();
                let jong = consonant_to_jongseong_index(
                    assemble_consonant((phoneme_codes[3], phoneme_codes[4])).unwrap(),
                )
                .unwrap();
                Ok((44032 + (cho * 588) + (jung * 28) + jong) as u32)
            } else {
                Err(())
            }
        }
        _ => Err(()),
    }
}

pub fn assemble_chars(phoneme_codes_chars: Vec<Vec<u32>>) -> Result<Vec<u32>, ()> {
    phoneme_codes_chars
        .into_iter()
        .map(|phoneme_codes| assemble(phoneme_codes))
        .collect()
}

pub fn assemble_consonant(consonant_codes: (u32, u32)) -> Result<u32, ()> {
    match consonant_codes {
        (12593, 12613) => Ok(12595),
        (12596, 12616) => Ok(12597),
        (12601, 12593) => Ok(12602),
        (12601, 12609) => Ok(12603),
        (12601, 12610) => Ok(12604),
        (12601, 12613) => Ok(12605),
        (12601, 12620) => Ok(12606),
        (12601, 12621) => Ok(12607),
        (12601, 12622) => Ok(12608),
        (12610, 12613) => Ok(12612),
        _ => Err(()),
    }
}

pub fn assemble_vowel(vowel_codes: (u32, u32)) -> Result<u32, ()> {
    match vowel_codes {
        (12631, 12623) => Ok(12632),
        (12631, 12624) => Ok(12633),
        (12631, 12643) => Ok(12634),
        (12636, 12627) => Ok(12637),
        (12636, 12628) => Ok(12638),
        (12636, 12643) => Ok(12639),
        (12641, 12643) => Ok(12642),
        _ => Err(()),
    }
}

pub fn disassemble(syllable_code: u32) -> Result<Vec<u32>, ()> {
    if is_hangul_complete(syllable_code) {
        let code = syllable_code - 44032;
        let cho = (code / 28) / 21;
        let jung = (code / 28) % 21;
        let jong = code % 28;
        match (
            choseong_index_to_consonant(cho as usize),
            jungseong_index_to_vowel(jung as usize),
            jongseong_index_to_consonant(jong as usize),
        ) {
            (Some(cho), Some(jung), Some(jong)) => Ok(vec![cho, jung, jong]),
            (Some(cho), Some(jung), None) => Ok(vec![cho, jung]),
            _ => Err(()),
        }
    } else {
        Ok(vec![syllable_code])
    }
}

pub fn deep_disassemble(syllable_code: u32) -> Result<Vec<u32>, ()> {
    if is_hangul_complete(syllable_code) {
        let code = syllable_code - 44032;
        let cho = (code / 28) / 21;
        let jung = (code / 28) % 21;
        let jong = code % 28;
        match (
            choseong_index_to_consonant(cho as usize),
            jungseong_index_to_vowel(jung as usize),
            jongseong_index_to_consonant(jong as usize),
        ) {
            (Some(cho), Some(jung), Some(jong)) => {
                match (disassemble_vowel(jung), disassemble_consonant(jong)) {
                    (Ok((jung0, jung1)), Ok((jong0, jong1))) => Ok(vec![cho, jung0, jung1, jong0, jong1]),
                    (Ok((jung0, jung1)), Err(())) => Ok(vec![cho, jung0, jung1, jong]),
                    (Err(()), Ok((jong0, jong1))) => Ok(vec![cho, jung, jong0, jong1]),
                    (Err(()), Err(())) => Ok(vec![cho, jung, jong]),
                }
            }
            (Some(cho), Some(jung), None) => match disassemble_vowel(jung) {
                Ok((jung0, jung1)) => Ok(vec![cho, jung0, jung1]),
                Err(()) => Ok(vec![cho, jung]),
            },
            _ => Err(()),
        }
    } else {
        Ok(vec![syllable_code])
    }
}

pub fn disassemble_chars(syllable_code_chars: Vec<u32>) -> Result<Vec<Vec<u32>>, ()> {
    syllable_code_chars
        .into_iter()
        .map(|syllable_code| disassemble(syllable_code))
        .collect()
}

pub fn deep_disassemble_chars(syllable_code_chars: Vec<u32>) -> Result<Vec<Vec<u32>>, ()> {
    syllable_code_chars
        .into_iter()
        .map(|syllable_code| deep_disassemble(syllable_code))
        .collect()
}

pub fn disassemble_consonant(consonant_code: u32) -> Result<(u32, u32), ()> {
    match consonant_code {
        12595 => Ok((12593, 12613)),
        12597 => Ok((12596, 12616)),
        12602 => Ok((12601, 12593)),
        12603 => Ok((12601, 12609)),
        12604 => Ok((12601, 12610)),
        12605 => Ok((12601, 12613)),
        12606 => Ok((12601, 12620)),
        12607 => Ok((12601, 12621)),
        12608 => Ok((12601, 12622)),
        12612 => Ok((12610, 12613)),
        _ => Err(()),
    }
}

pub fn disassemble_vowel(vowel_code: u32) -> Result<(u32, u32), ()> {
    match vowel_code {
        12632 => Ok((12631, 12623)),
        12633 => Ok((12631, 12624)),
        12634 => Ok((12631, 12643)),
        12637 => Ok((12636, 12627)),
        12638 => Ok((12636, 12628)),
        12639 => Ok((12636, 12643)),
        12642 => Ok((12641, 12643)),
        _ => Err(()),
    }
}

pub fn is_consonant(code: u32) -> bool {
    match code {
        12593..=12622 => true,
        _ => false,
    }
}

pub fn is_simple_consonant(code: u32) -> bool {
    match code {
        12593..=12594 | 12596 | 12598..=12601 | 12609..=12611 | 12613..=12622 => true,
        _ => false,
    }
}

pub fn is_complex_consonant(code: u32) -> bool {
    match code {
        12595 | 12597 | 12602..=12608 | 12612 => true,
        _ => false,
    }
}

pub fn is_vowel(code: u32) -> bool {
    match code {
        12623..=12643 => true,
        _ => false,
    }
}

pub fn is_simple_vowel(code: u32) -> bool {
    match code {
        12623..=12631 | 12635..=12636 | 12640..=12641 | 12643 => true,
        _ => false,
    }
}

pub fn is_complex_vowel(code: u32) -> bool {
    match code {
        12632..=12634 | 12637..=12639 | 12642 => true,
        _ => false,
    }
}

pub fn is_choseong(code: u32) -> bool {
    match code {
        12593 | 12594 | 12596 | 12599..=12601 | 12609..=12611 | 12613..=12622 => true,
        _ => false,
    }
}

pub fn is_jungseong(code: u32) -> bool {
    match code {
        12623..=12643 => true,
        _ => false,
    }
}

pub fn is_jongseong(code: u32) -> bool {
    match code {
        12593..=12599 | 12601..=12610 | 12612..=12616 | 12618..=12622 => true,
        _ => false,
    }
}

pub fn is_hangul(code: u32) -> bool {
    match code {
        12593..=12643 | 44032..=55203 => true,
        _ => false,
    }
}

pub fn is_hangul_complete(code: u32) -> bool {
    match code {
        44032..=55203 => true,
        _ => false,
    }
}

fn consonant_to_choseong_index(consonant_code: u32) -> Option<usize> {
    match consonant_code - 12593 {
        0 => Some(0),
        1 => Some(1),
        3 => Some(2),
        6 => Some(3),
        7 => Some(4),
        8 => Some(5),
        16 => Some(6),
        17 => Some(7),
        18 => Some(8),
        20 => Some(9),
        21 => Some(10),
        22 => Some(11),
        23 => Some(12),
        24 => Some(13),
        25 => Some(14),
        26 => Some(15),
        27 => Some(16),
        28 => Some(17),
        29 => Some(18),
        _ => None,
    }
}

fn consonant_to_jongseong_index(consonant_code: u32) -> Option<usize> {
    match consonant_code - 12593 {
        0 => Some(1),
        1 => Some(2),
        2 => Some(3),
        3 => Some(4),
        4 => Some(5),
        5 => Some(6),
        6 => Some(7),
        8 => Some(8),
        9 => Some(9),
        10 => Some(10),
        11 => Some(11),
        12 => Some(12),
        13 => Some(13),
        14 => Some(14),
        15 => Some(15),
        16 => Some(16),
        17 => Some(17),
        19 => Some(18),
        20 => Some(19),
        21 => Some(20),
        22 => Some(21),
        23 => Some(22),
        25 => Some(23),
        26 => Some(24),
        27 => Some(25),
        28 => Some(26),
        29 => Some(27),
        _ => None,
    }
}

fn vowel_to_jungseong_index(vowel_code: u32) -> Option<usize> {
    if is_jungseong(vowel_code) {
        Some((vowel_code - 12623) as usize)
    } else {
        None
    }
}

fn choseong_index_to_consonant(choseong_index: usize) -> Option<u32> {
    match choseong_index {
        0 => Some(12593),
        1 => Some(12594),
        2 => Some(12596),
        3 => Some(12599),
        4 => Some(12600),
        5 => Some(12601),
        6 => Some(12609),
        7 => Some(12610),
        8 => Some(12611),
        9 => Some(12613),
        10 => Some(12614),
        11 => Some(12615),
        12 => Some(12616),
        13 => Some(12617),
        14 => Some(12618),
        15 => Some(12619),
        16 => Some(12620),
        17 => Some(12621),
        18 => Some(12622),
        _ => None,
    }
}

fn jungseong_index_to_vowel(jungseong_index: usize) -> Option<u32> {
    if is_vowel((jungseong_index + 12623) as u32) {
        Some((jungseong_index + 12623) as u32)
    } else {
        None
    }
}

fn jongseong_index_to_consonant(jongseong_index: usize) -> Option<u32> {
    match jongseong_index {
        1 => Some(12593),
        2 => Some(12594),
        3 => Some(12595),
        4 => Some(12596),
        5 => Some(12597),
        6 => Some(12598),
        7 => Some(12599),
        8 => Some(12601),
        9 => Some(12602),
        10 => Some(12603),
        11 => Some(12604),
        12 => Some(12605),
        13 => Some(12606),
        14 => Some(12607),
        15 => Some(12608),
        16 => Some(12609),
        17 => Some(12610),
        18 => Some(12612),
        19 => Some(12613),
        20 => Some(12614),
        21 => Some(12615),
        22 => Some(12616),
        23 => Some(12618),
        24 => Some(12619),
        25 => Some(12620),
        26 => Some(12621),
        27 => Some(12622),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::hangul;

    #[test]
    fn it_works() {
        assert_eq!(
            hangul::assemble(hangul::disassemble(45285).unwrap()).unwrap(),
            45285
        );
        assert_eq!(
            hangul::assemble(hangul::deep_disassemble(48577).unwrap()).unwrap(),
            48577
        );
    }
}
