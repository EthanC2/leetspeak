mod translation_tables;

use crate::translation_tables::{LEETSPEAK_TABLE_LEVEL1, LEETSPEAK_TABLE_LEVEL2, LEETSPEAK_TABLE_LEVEL3, LEETSPEAK_TABLE_COMPLETE};
use std::collections::HashMap;
use rand::prelude::*;

/// Translates `text` into leetspeak, returning a new string. Since each English letter
/// is mapped to multiple leetspeak letters, each leetspeak letter is chosen from its mapping at random using
/// [`rand::thread_rng()`]. This means that the result of [`translate()`] is non-deterministic
/// (i.e. translating the same `text` may return different results). If you want deterministic (non-random)
/// translation, use [`translate_with_level()`] or [`translate_custom()`]. 
/// The translation table is based on [wikipedia/leet](https://en.wikipedia.org/wiki/Leet#Orthography)
///     
/// Usage: 
/// ```
/// let text = "sphinx of black quartz, judge my vow";
/// let translation = leetspeak::translate(text);
/// ```
pub fn translate<S: AsRef<str>>(text: S) -> String {
    let mut translation = String::new();
    let mut rng = thread_rng();

    text.as_ref()
        .chars()
        .for_each(|ch| {
            if let Some(mapping) = LEETSPEAK_TABLE_COMPLETE.get(&ch.to_ascii_lowercase()) {
                translation.push_str(mapping.choose(&mut rng).expect("no array in the table is empty"));
            } else {
                translation.push(ch);
            }
        });

    translation
}

/// The level determines the degree of translation. [`Level::One`] replaces a few common
/// letters with numbers;  [`Level::Two`] replaces most letters with either single-digit numbers 
/// multi-character strings that use symbols to represent characters; [`Level::Three`] is the same
/// as level 2, except it replaces *all* letters in the original text. Below are examples of the 
/// translation output using the pangram "sphinx of black quartz, judge my vow"
/// 
/// | Level | Output | 
/// | ----- | ------ |
/// | Level 1: | "5ph1nx 0f 814ck qu427z, jud93 my v0w" |
/// | Level 2: | "5p#1nx 0f 81@<k qv@27z, \_\|v\|)93 m`/ \\/0vv" |
/// | Level 3: | "$\|>/-/!\|\\\|}{ ()/= /3\|\_@(\|< 0\_v@I2+7\_, \_]vcl(\_+& /V\\`/ \\\|()vv" |
pub enum Level {
    One,
    Two,
    Three
}

/// Translates `text` into leetspeak using the translation level `level`, returning a new string.
/// [`Level::One`] replaces a few common letters with numbers; [`Level::Two`] replaces most letters with 
/// either single-digit numbers multi-character strings that use symbols to represent characters; 
/// [`Level::Three`] is the same as level 2, except it replaces all letters in the original text. 
/// 
/// Usage:
/// ```
/// let text = "sphinx of black quartz, judge my vow";
/// let translation = leetspeak::translate_with_level(text, leetspeak::Level::One);
/// assert_eq!(translation, r#"5ph1nx 0f 814ck qu427z, jud93 my v0w"#);
/// ```
pub fn translate_with_level<S: AsRef<str>>(text: S, level: Level) -> String {
    let mut translation = String::new();
    let mapping = match level {
        Level::One => LEETSPEAK_TABLE_LEVEL1,
        Level::Two =>  LEETSPEAK_TABLE_LEVEL2,
        Level::Three => LEETSPEAK_TABLE_LEVEL3,
    };

    text.as_ref()
        .chars()
        .for_each(|ch| 
            if let Some(string) = mapping.get(&ch.to_ascii_lowercase()) {
                translation.push_str(string);
            } else {
                translation.push(ch);
            }
        );

    translation
}

/// Translates `text` into leetspeak using a custom mapping table (type `HashMap<char,String>`),
/// returning a new string. Characters not included in the mapping table are not changed.
/// 
/// Usage:
/// ```
/// let mapping = std::collections::HashMap::from([
///    ('a', String::from("4")),
///    ('c', String::from("<")),
///    ('e', String::from("€")),
///    ('m', String::from(r#"/\/\"#)),
///    ('p', String::from("|*")),
///    ('s', String::from("ehs")),
///    ('w', String::from("vv")),
///    ('z', String::from("7_")),
///]);
///
///    let text = "sphinx of black quartz, judge my vow";
///    let translation = leetspeak::translate_custom(text, &mapping, &true);
///    assert_eq!(translation, r#"ehs|*hinx of bl4<k qu4rt7_, judg€ /\/\y vovv"#);
///```
pub fn translate_custom<S: AsRef<str>>(text: S, mapping: &HashMap<char,String>, case_sensitive: &bool) -> String {
    let mut translation = String::new();
    
    text.as_ref()
        .chars()
        .for_each(|ch| {
            if let Some(string) = mapping.get(&ch) {
                translation.push_str(string);
            } else if !*case_sensitive {
                if let Some(string) = mapping.get(&invert_ascii_case(&ch)) {
                    translation.push_str(string);
                }
            } else {
                translation.push(ch);
            }
        });

    translation
}

/// Inverts the case of an ASCII character.
/// 'A' => 'a' and 'a' => 'A'
#[inline(always)]
fn invert_ascii_case(ch: &char) -> char {
    if ch.is_ascii_lowercase() {
        ch.to_ascii_uppercase()
    } else {
        ch.to_ascii_lowercase()
    }
}