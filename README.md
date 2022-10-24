# leetspeak
*leetspeak* is a leetspeak translation library with support for random, non-random, 
and custom leetspeak translation. Translations are based on [wikipedia/leet#orthography](https://en.wikipedia.org/wiki/Leet#Orthography) 
Using this library is as simple as:

```rs
let text = "sphinx of black quartz, judge my vow";

// Random leetspeak translation
let random_translation = leetspeak::translate(text);

// Non-random leetspeak translation, levels 1-3
let nonrandom_translation = leetspeak::translate_with_level(text, Level::One);
assert_eq!(nonrandom_translation, r#"5ph1nx 0f 814ck qu427z, jud93 my v0w"#);

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

let custom_translation = leetspeak::translate_custom(text, mapping);
assert_eq!(custom_translation, r#"ehs|*hinx of bl4<k qu4rt7_, judg€ /\/\y vovv"#);
```

# Contributing
This is an open-source project and contributors are welcome!
The repository is at [github.com: leetspeak](https://github.com/EthanC2/leetspeak) 
and development plans/progress are documented at [github.com: leetspeak/projects](https://github.com/users/EthanC2/projects/5/views/1). If this library is missing a feature you would like, 
feel free to make a suggestion.