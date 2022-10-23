mod translation_tables;

use std::collections::HashMap;
use rand::prelude::*;
use crate::translation_tables::{LEETSPEAK_TABLE_LEVEL1, LEETSPEAK_TABLE_LEVEL2, LEETSPEAK_TABLE_LEVEL3, LEETSPEAK_TABLE_COMPLETE};

pub fn translate(text: &str) -> String {
    let mut rng = thread_rng();

    text.chars()
        .into_iter()
        .fold(String::from(""), |accum, c|
            match LEETSPEAK_TABLE_COMPLETE.get(&c) {
                Some(mapping) => accum + *mapping.choose(&mut rng)
                                                           .expect("no array in the table is empty"),
                None => accum + c.to_string().as_str()
            }
    )
}


pub enum Level {
    One,
    Two,
    Three
}

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
            match translation_table.get(&c) {
                Some(s) => accum + *s,
                None => accum + c.to_string().as_str()
            }
        )
}

pub fn translate_custom(text: &str, table: HashMap<char,String>) -> String {
    text.chars()
        .into_iter()
        .fold(String::from(""), |accum, c| 
        match table.get(&c) {
                Some(s) => accum + s,
                None => accum + c.to_string().as_str()
            }
        )
}