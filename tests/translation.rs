use leetspeak::Level;

#[test]
fn translate_default() {
    /*
        Since `leetspeak::translate()` is a non-deterministic function, it
        cannot be tested using static data. So, I've just validated that it 
        does have *some* output. There has to be some way to test it though. todo!().
    */
    let text = "sphinx of black quartz, judge my vow";
    let translation = leetspeak::translate(text);
    assert!(!translation.is_empty());
}

#[test]
fn translate_level1() {
    let text = "sphinx of black quartz, judge my vow";
    let translation = leetspeak::translate_with_level(text, Level::One);
    assert_eq!(translation, r#"5ph1nx 0f 814ck qu427z, jud93 my v0w"#);
}

#[test]
fn translate_level2() {
    let text = "sphinx of black quartz, judge my vow";
    let translation = leetspeak::translate_with_level(text, Level::Two);
    assert_eq!(translation, r#"5p#1nx 0f 81@<k qv@27z, _|v|)93 m`/ \/0vv"#);
}

#[test]
fn translate_level3() {
    let text = "sphinx of black quartz, judge my vow";
    let translation = leetspeak::translate_with_level(text, Level::Three);
    assert_eq!(translation, r#"$|>/-/!|\|}{ ()/= /3|_@(|< 0_v@I2+7_, _]vcl(_+& /V\`/ \|()vv"#);
}

/// Built-in translate functions implicitly cast to ASCII lowercase so translations are 
/// case-insensitive. This function verifies is proof.
#[test]
fn case_insensitivity() {
    let lhs = leetspeak::translate_with_level("sphinx of black quartz, judge my vow", Level::Three);
    let rhs = leetspeak::translate_with_level("SPHINX OF BLACK QUARTZ, JUDGE MY VOW", Level::Three);
    assert_eq!(lhs, rhs);
}