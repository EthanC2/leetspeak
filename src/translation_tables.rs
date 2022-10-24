use phf::phf_map;


/* 
    Level 1: 
    Basic leetspeak replaces some letters with single-digit numbers
*/
pub static LEETSPEAK_TABLE_LEVEL1: phf::Map<char,&'static str> = phf_map!(
    'a' => "4",
    'b' => "8",
    'e' => "3",
    'g' => "9",
    'i' => "1",
    'l' => "1",
    'o' => "0",
    'r' => "2",
    's' => "5",
    't' => "7",
);

/* 
    Level 2:
    Intermediate leekspeak replaces most letters; the replaced letters
    are substituted for either single-digit numbers multi-character strings that 
    use symbols to represent characters. For example, `/ can be used to represent 
    the letter y

    I left out some of the weird/goth letters like 'q' and 'z'
    cause they're cool
*/
pub static LEETSPEAK_TABLE_LEVEL2: phf::Map<char,&'static str> = phf_map!(
    'a' => "@",
    'b' => "8",
    'c' => "<",
    'd' => "|)",
    'e' => "3",
    'g' => "9",
    'h' => "#",
    'i' => "1",
    'j' => "_|",
    'k' => "k",
    'l' => "1",
    'o' => "0",
    'r' => "2",
    's' => "5",
    't' => "7",
    'u' => "v",
    'v' => "\\/",
    'w' => "vv",
    'y' => "`/",
);

/* 
    Level 3:
    Full leekspeak replaces all letters; the replaced letters
    are substituted for either single-digit numbers multi-character strings that 
    use symbols to represent characters. For example, '][' can be used to represent 
    the letter T

    Good luck reading this.
*/
pub static LEETSPEAK_TABLE_LEVEL3: phf::Map<char,&'static str> = phf_map!(
    'a' => "@",
    'b' => "/3",
    'c' => "(",
    'd' => "cl",
    'e' => "&",
    'f' => "/=",
    'g' => "(_+",
    'h' => "/-/",
    'i' => "!",
    'j' => "_]",
    'k' => "|<",
    'l' => "|_",
    'm' => "/V\\",
    'n' => "|\\|",
    'o' => "()",
    'p' => "|>",
    'q' => "0_",
    'r' => "I2",
    's' => "$",
    't' => "+",
    'u' => "v",
    'v' => "\\|",
    'w' => "vv",
    'x' => "}{",
    'y' => "`/",
    'z' => "7_",
);

/*
    A complete table of character-to-symbol mapping, based on [Wikipedia: Leet](https://en.wikipedia.org/wiki/Leet)
*/
pub static LEETSPEAK_TABLE_COMPLETE: phf::Map<char,&'static [&'static str]> = phf_map!(
    'a' => &["4", r#"/\"#, "@", r#"/-\"#, "^", "(L", "Д"],
    'b' => &["I3", "8", "13", "|3", "ß", "!3", "(3", "/3", ")3", "|-]", "j3"],
    'c' => &["[", "¢", "<", "(", "©"],
    'd' => &[")", "|)", "(|", "[)", "I>", "|>", "T)", "I7", "cl", "|}", "|]"],
    'e' => &["3", "&", "£", "€", "[-", "|=-"],
    'f' => &["|=", "ƒ", "|#", "ph", "/=", "v"],
    'g' => &["6", "&", "(_+", "9", "C-", "gee", "(?,", "[,", "{,", "<-", "(."],
    'h' => &["#", "/-/", r#"\-\"#, "[-]", "]-[", ")-(", "(-)", ":-:", "|~|", "|-|", "]~[", "}{", "!-!", "1-1", r#"\-/"#, "I+I"],
    'i' => &["1", "|", "][", "!", "eye", "3y3"],
    'j' => &[",_|", "_|", "._|", "._]", "_]", ",_]", "]"],
    'k' => &[">|", "|<", "1<", "|c", "|(", "7c"],
    'l' => &["1", "2", "£", "7", "|_", "|"],
    'm' => &[r#"/\/\"#, r#"/V\"#, "[V]", r#"|\/|"#, "^^", r#"<\/>"#, "{V}", "(v)", "(V)", r#"|\|\"#, r#"]\/["#, "nn", "11"],
    'n' => &["^/", r#"|\|"#, r#"/\/"#, r#"[\]"#, r#"<\>"#, r#"{\}"#, r#"/V"#, "^", "ท"],
    'o' => &["0", "()", "oh", "[]", "p", "<>", "Ø"],
    'p' => &["|*", "|o", "|º", "|^", "|>", "|\"", "9", "[]D", "|7"],
    'q' => &["(_,)", "()_", "2", "0_", "<|", "&", "9"],
    'r' => &["I2", "9", "|`", "|~", "|?", "/2", "|^", "lz", "7", "2", "12", "®", "[z", "Я", ".-", "|2", "|-", "3"],
    's' => &["5", "$", "z", "§", "ehs", "es", "2"],
    't' => &["7", "+", "-|-", r#"']['"#, "†", "«|»", "~|~"],
    'u' => &["(_)", "|_|", "v", "L|", "บ"],
    'v' => &[r#"\/"#, "|/", r#"\|"#],
    'w' => &[r#"\/\/"#, "vv", r#"\N"#, r#"'//"#, r#"\\'"#, r#"\^/"#, "dubya", "(n)", r#"\V/"#, r#"\X/"#, r#"\|/"#, r#"\_|_/"#, r#"\_:_/"#, "uu", "2u", r#"\\//\\//"#, "พ"],
    'x' => &["><", "}{", "ecks", "×", "?", ")(", "]["],
    'y' => &["j", "`/", r#"\|/"#, "¥", r#"\//"#],
    'z' => &["2", "7_", "-/_", "%", ">_", "s", "~/_", r#"-\_"#, "-|_"],
);