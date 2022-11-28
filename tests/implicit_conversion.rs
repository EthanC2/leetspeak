/*
    Tests `leetspeak::translate()`, `leetspeak::translate_with_level()`,
    and `leetspeak::translate_custom()`'s implicit conversion of String,
    &String, and &str to &str via AsRef<str>. All methods use the same
    generic, so only testing one is required.
*/

#[test]
fn with_string() {
    let string = String::from("ferris");
    let _translation = leetspeak::translate(string);
}

#[test]
fn with_string_ref() {
    let string_ref = &String::from("my");
    let _translation = leetspeak::translate(string_ref);
}

#[test]
fn with_str() {
    let str = "beloved";
    let _translation = leetspeak::translate(str);
}