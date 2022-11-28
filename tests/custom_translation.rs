/// Verifies that the custom translation works as intended.
/// Input: A plaintext string and a mapping of chars to symbols (String)
/// Output: A transformed plaintext string, translated with the given mapping
#[test]
fn translate() {
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
    let translation = leetspeak::translate_custom(text, &mapping, &true);
    assert_eq!(translation, r#"ehs|*hinx of bl4<k qu4rt7_, judg€ /\/\y vovv"#);
}

/// Case-sensitive custom translations should ONLY translate the characters
/// found in the provided mapping. Any other characters should remain unchanged.
#[test]
fn case_sensitive() {
    let lhs_text = "ABCDEF";
    let rhs_text = "abcdef";
    let mapping = std::collections::HashMap::from([
        ('A',String::from(r#"/-\"#)),
        ('b',String::from("|3")),
        ('C',String::from("<")),
        ('d',String::from("I7")),
        ('E',String::from("€")),
        ('f',String::from("ƒ")),
    ]);

    let lhs_translation = leetspeak::translate_custom(lhs_text, &mapping, &true);
    let rhs_translation = leetspeak::translate_custom(rhs_text, &mapping, &true);
    assert_eq!(lhs_translation, r#"/-\B<D€F"#);
    assert_eq!(rhs_translation, r#"a|3cI7eƒ"#);
    assert_ne!(lhs_translation, rhs_translation);
}

/// Case insensitive custom translations should only map characters in provided mapping. If
/// a character is not in the mapping, but can be ASCII case-transformed such that it would be in the
/// mapping, then that character should be translated to its case-transformed counterpart's mapping.
#[test]
fn case_insensitive() {
    let lhs_text = "sphinx of black quartz, judge my vow";
    let rhs_text = "SPHINX OF BLACK QUARTZ, JUDGE MY VOW";
    let mapping = std::collections::HashMap::from([
        ('a', String::from(r#"/-\"#)),
        ('b',String::from("8")),
        ('c',String::from("<")),
        ('d',String::from("|)")),
        ('e',String::from("3")),
        ('f',String::from("/=")),
        ('g',String::from("(_+")),
        ('h',String::from("/-/")),
        ('i',String::from("!")),
        ('j',String::from(",_]")),
        ('k',String::from("|<")),
        ('l',String::from("|_")),
        ('m',String::from("^^")),
        ('n',String::from("ท")),
        ('o',String::from("<>")),
        ('p',String::from("|º")),
        ('q',String::from("0_")),
        ('r',String::from("Я")),
        ('s',String::from("§")),
        ('t',String::from("†")),
        ('u',String::from("|_|")),
        ('v',String::from("|/")),
        ('w',String::from("(n)")),
        ('x',String::from("×")),
        ('y',String::from("¥")),
        ('z',String::from(r#"-\_"#)),
    ]);

    let lhs_translation = leetspeak::translate_custom(lhs_text, &mapping, &false);
    let rhs_translation = leetspeak::translate_custom(rhs_text, &mapping, &false);
    assert_eq!(lhs_translation, rhs_translation);
}

/// Case-insensitve custom translations should prioritize mapping a character without transforming it
/// before attempting case-transformation.
#[test]
fn prioritize_sensitive_mapping() {
    let text = "SaY";
    let mapping = std::collections::HashMap::from([
        ('S', String::from("$")),
        ('s', String::from("5")),
        ('A', String::from(r#"/-\"#)),
        ('a', String::from("4")),
        ('Y', String::from("`/")),
        ('y', String::from("¥")),
    ]);

    let translation = leetspeak::translate_custom(text, &mapping, &false);
    assert_eq!(translation, "$4`/");
}