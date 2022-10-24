# leetspeak
*leetspeak* is a leetspeak translation library with support for random, non-random, 
and custom leetspeak translation. Using this library is as simple as.

```rs
let text = "sphinx of black quartz, judge my vow";
let translation = leetspeak::translate(text);
```

If you want to use a custom leetspeak mapping, use `leetspeak::translate_custom()`
with a mapping (type `HashMap<char,String>`).