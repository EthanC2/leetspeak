# leetspeak
*leetspeak* is a leetspeak translation library with support for random, non-random, 
and custom leetspeak translation. Using this library is as simple as:

```rs
let text = "sphinx of black quartz, judge my vow";

// Random leetspeak translation
let translation = leetspeak::translate(text);

// Non-random leetspeak translation, levels 1-3
let translation = leetspeak::translate_with_level(text, Level::One);
assert_eq!(translation, r#"5ph1nx 0f 814ck qu427z, jud93 my v0w"#);

//Custom leetspeak translation
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

let translation = leetspeak::translate_custom(text, mapping);
assert_eq!(translation, r#"ehs|*hinx of bl4<k qu4rt7_, judg€ /\/\y vovv"#);
```
