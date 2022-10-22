mod translation_tables;

use std::collections::HashMap;
use rand::prelude::*;
use crate::translation_tables::{LEVEL1, LEVEL2, LEVEL3, LEETSPEAK_TABLE_COMPLETE};

/*
    IMPL FOR LEVELERROR
*/
pub struct LevelError;

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

pub fn translate_with_level(text: &str, level: &u8) -> Result<String, LevelError> {
    let translation_table;
    match level {
        1 => translation_table = LEVEL1,
        2 => translation_table = LEVEL2,
        3 => translation_table = LEVEL3,
        _ => return Err(LevelError)
    }

    Ok(text.chars()
        .into_iter()
        .fold(String::from(""), |accum, c| 
            match translation_table.get(&c) {
                Some(s) => accum + *s,
                None => accum + c.to_string().as_str()
            }
        ))
}

pub fn translate_with_custom(text: &str, table: HashMap<char,&str>) -> String {
    text.chars()
        .into_iter()
        .fold(String::from(""), |accum, c| 
        match table.get(&c) {
                Some(s) => accum + *s,
                None => accum + c.to_string().as_str()
            }
        )
}