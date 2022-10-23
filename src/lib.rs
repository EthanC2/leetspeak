mod translation_tables;

use std::collections::HashMap;
use rand::prelude::*;
use crate::translation_tables::{LEETSPEAK_TABLE_LEVEL1, LEETSPEAK_TABLE_LEVEL2, LEETSPEAK_TABLE_LEVEL3, LEETSPEAK_TABLE_COMPLETE};

/// Translates `text` into leetspeak, returning a new string. Since each English letter
/// is mapped to multiple leetspeak letters, each leetspeak letter is chosen from its mapping at random using
/// `rand::thread_rng()`. This means that the result of `leetspeak::translate()` is non-deterministic
/// (i.e. translating the same `text` may return different results). If you want deterministic (non-random)
/// translation, use `leetspeak::translate_with_level()` or `leetspeak::translate_custom()`. 
/// The translation table is based on [wikipedia/leet](https://en.wikipedia.org/wiki/Leet#Orthography)
///     
/// Usage: 
/// ```
/// let text = "sphinx of black quartz, judge my vow";
/// let translation = leetspeak::translate(text);
/// ```
pub fn translate(text: &str) -> String {
    let mut rng = thread_rng();

    text.chars()
        .into_iter()
        .fold(String::from(""), |accum, c|
            match LEETSPEAK_TABLE_COMPLETE.get(&c.to_ascii_lowercase()) {
                Some(mapping) => accum + *mapping.choose(&mut rng)
                                                           .expect("no array in the table is empty"),
                None => accum + c.to_string().as_str()
        })
}

/// The level determines the degree of translation. `Level::One` replaces a few common
/// letters with numbers;  `Level::Two` replaces most letters with either single-digit numbers 
/// multi-character strings that use symbols to represent characters; `Level::Three` is the same
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
/// Level::One replaces a few common letters with numbers; Level::Two replaces most letters with 
/// either single-digit numbers multi-character strings that use symbols to represent characters; 
/// Level::Three is the same as level 2, except it replaces all letters in the original text. 
/// 
/// Usage:
/// ```
/// let text = "sphinx of black quartz, judge my vow";
/// let translation = leetspeak::translate_with_level(text, leetspeak::Level::One);
/// assert_eq!(translation, r#"5ph1nx 0f 814ck qu427z, jud93 my v0w"#);
/// ```
pub fn translate_with_level(text: &str, level: Level) -> String {
    let translation_table;
    match level {
        Level::One => translation_table = LEETSPEAK_TABLE_LEVEL1,
        Level::Two => translation_table = LEETSPEAK_TABLE_LEVEL2,
        Level::Three => translation_table = LEETSPEAK_TABLE_LEVEL3,
    }

    text.chars()
        .into_iter()
        .fold(String::from(""), |accum, c| 
            match translation_table.get(&c.to_ascii_lowercase()) {
                Some(s) => accum + *s,
                None => accum + c.to_string().as_str(),
            }
        )
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
///    let translation = leetspeak::translate_custom(text, mapping);
///    assert_eq!(translation, r#"ehs|*hinx of bl4<k qu4rt7_, judg€ /\/\y vovv"#);
///```
pub fn translate_custom(text: &str, table: HashMap<char,String>) -> String {
    text.chars()
        .into_iter()
        .fold(String::from(""), |accum, c| 
        match table.get(&c.to_ascii_lowercase()) {
                Some(s) => accum + s,
                None => accum + c.to_string().as_str(),
            }
        )
}