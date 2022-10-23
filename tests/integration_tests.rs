use leetspeak::Level;

#[test]
fn translate_random() {
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

#[test]
fn translate_custom() {
    let mapping = std::collections::HashMap::from([
        ('a', String::from("4")),
        ('c', String::from("<")),
        ('e', String::from("€")),
        ('m', String::from(r#"/\/\"#)),
        ('p', String::from("|*")),
        ('s', String::from("ehs")),
        ('w', String::from("vv")),
        ('z', String::from("7_")),
    ]);

    let text = "sphinx of black quartz, judge my vow";
    let translation = leetspeak::translate_custom(text, mapping);
    assert_eq!(translation, r#"ehs|*hinx of bl4<k qu4rt7_, judg€ /\/\y vovv"#);
}