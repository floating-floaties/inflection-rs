#![forbid(unsafe_code)]
#![allow(dead_code)]

use std::collections::HashSet;
use regex::Regex;
use lazy_static::lazy_static;

macro_rules! case_insensitive {
    ($str:expr) => {{
        $str
            .chars()
            .map(|c| format!("[{}{}]", c.to_string(), c.to_uppercase().to_string()))
            .collect::<String>()
    }};
}

type Upsc = (HashSet<String>, Vec<(Regex, String)>, Vec<(Regex, String)>, Vec<Regex>);
lazy_static! {
    static ref UPS: Upsc = {
        let mut plurals: Vec<(String, String)> = vec![
                (r"(?i)(?P<a>\w+)s?-in-law$".to_string(), "${a}s-in-law".to_string()),
                (r"(?i)(?P<a>quiz)$".to_string(), "${a}zes".to_string()),
                (r"(?i)^(?P<a>oxen)$".to_string(), "${a}".to_string()),
                (r"(?i)^(?P<a>ox)$".to_string(), "${a}en".to_string()),
                (r"(?i)(?P<a>m|l)ice$".to_string(), "${a}ice".to_string()),
                (r"(?i)(?P<a>m|l)ouse$".to_string(), "${a}ice".to_string()),
                (r"(?i)(?P<a>passer)s?by$".to_string(), "${a}sby".to_string()),
                (
                    r"(?i)(?P<a>matr|vert|ind)(?:ix|ex)$".to_string(),
                    "${a}ices".to_string(),
                ),
                (r"(?i)(?P<a>x|ch|ss|sh)$".to_string(), "${a}es".to_string()),
                (
                    r"(?i)(?P<a>[^aeiouy]|qu)y$".to_string(),
                    "${a}ies".to_string(),
                ),
                (r"(?i)(?P<a>hive)$".to_string(), "${a}s".to_string()),
                (r"(?i)(?P<a>[lr])f$".to_string(), "${a}ves".to_string()),
                (r"(?i)(?P<a>[^f])fe$".to_string(), "${a}ves".to_string()),
                (r"(?i)sis$".to_string(), "ses".to_string()),
                (r"(?i)(?P<a>[ti])a$".to_string(), "${a}a".to_string()),
                (r"(?i)(?P<a>[ti])um$".to_string(), "${a}a".to_string()),
                (
                    r"(?i)(?P<a>buffal|potat|tomat|her)o$".to_string(),
                    "${a}oes".to_string(),
                ),
                (r"(?i)(?P<a>bu)s$".to_string(), "${a}ses".to_string()),
                (
                    r"(?i)(?P<a>alias|status)$".to_string(),
                    "${a}es".to_string(),
                ),
                (r"(?i)(?P<a>octop|vir|radi)i$".to_string(), "${a}i".to_string()),
                (r"(?i)(?P<a>octop|vir|radi)us$".to_string(), "${a}i".to_string()),
                (r"(?i)^(?P<a>ax|test)is$".to_string(), "${a}es".to_string()),
                (r"(?i)s$".to_string(), "s".to_string()),
                (r"$".to_string(), "s".to_string()),
            ];

            let mut singulars: Vec<(String, String)> = vec![
                (r"(?i)(?P<a>\w+)s-in-law$".to_string(), "${a}-in-law".to_string()),
                (r"(?i)(?P<a>database)s$".to_string(), "${a}".to_string()),
                (r"(?i)(?P<a>quiz)zes$".to_string(), "${a}".to_string()),
                (r"(?i)(?P<a>matr)ices$".to_string(), "${a}ix".to_string()),
                (
                    r"(?i)(?P<a>vert|ind)ices$".to_string(),
                    "${a}ex".to_string(),
                ),
                (r"(?i)(?P<a>passer)sby$".to_string(), "${a}by".to_string()),
                (r"(?i)^(?P<a>ox)en".to_string(), "${a}".to_string()),
                (
                    r"(?i)(?P<a>alias|status)(es)?$".to_string(),
                    "${a}".to_string(),
                ),
                (
                    r"(?i)(?P<a>octop|vir|radi)(us|i)$".to_string(),
                    "${a}us".to_string(),
                ),
                (r"(?i)^(?P<a>a)x[ie]s$".to_string(), "${a}xis".to_string()),
                (
                    r"(?i)(?P<a>cris|test)(is|es)$".to_string(),
                    "${a}is".to_string(),
                ),
                (r"(?i)(?P<a>shoe)s$".to_string(), "${a}".to_string()),
                (r"(?i)(?P<a>o)es$".to_string(), "${a}".to_string()),
                (r"(?i)(?P<a>bus)(es)?$".to_string(), "${a}".to_string()),
                (r"(?i)(?P<a>m|l)ice$".to_string(), "${a}ouse".to_string()),
                (r"(?i)(?P<a>x|ch|ss|sh)es$".to_string(), "${a}".to_string()),
                (r"(?i)(?P<a>m)ovies$".to_string(), "${a}ovie".to_string()),
                (r"(?i)(?P<a>s)eries$".to_string(), "${a}eries".to_string()),
                (
                    r"(?i)(?P<a>[^aeiouy]|qu)ies$".to_string(),
                    "${a}y".to_string(),
                ),
                (r"(?i)(?P<a>[lr])ves$".to_string(), "${a}f".to_string()),
                (r"(?i)(?P<a>tive)s$".to_string(), "${a}".to_string()),
                (r"(?i)(?P<a>hive)s$".to_string(), "${a}".to_string()),
                (r"(?i)(?P<a>[^f])ves$".to_string(), "${a}fe".to_string()),
                (
                    r"(?i)(?P<a>t)he(sis|ses)$".to_string(),
                    "${a}hesis".to_string(),
                ),
                (
                    r"(?i)(?P<a>s)ynop(sis|ses)$".to_string(),
                    "${a}ynopsis".to_string(),
                ),
                (
                    r"(?i)(?P<a>p)rogno(sis|ses)$".to_string(),
                    "${a}rognosis".to_string(),
                ),
                (
                    r"(?i)(?P<a>p)arenthe(sis|ses)$".to_string(),
                    "${a}arenthesis".to_string(),
                ),
                (
                    r"(?i)(?P<a>d)iagno(sis|ses)$".to_string(),
                    "${a}iagnosis".to_string(),
                ),
                (
                    r"(?i)(?P<a>b)a(sis|ses)$".to_string(),
                    "${a}asis".to_string(),
                ),
                (
                    r"(?i)(?P<a>a)naly(sis|ses)$".to_string(),
                    "${a}nalysis".to_string(),
                ),
                (r"(?i)(?P<a>[ti])a$".to_string(), "${a}um".to_string()),
                (r"(?i)(?P<a>n)ews$".to_string(), "${a}ews".to_string()),
                (r"(?i)(?P<a>ss)$".to_string(), "${a}".to_string()),
                (r"(?i)s$".to_string(), "".to_string()),
            ];

        let uncountable = HashSet::<String>::from([
            "equipment".to_string(),
            "fish".to_string(),
            "information".to_string(),
            "jeans".to_string(),
            "money".to_string(),
            "rice".to_string(),
            "series".to_string(),
            "sheep".to_string(),
            "species".to_string(),
        ]);

        let uncountable_progs: Vec<Regex> = uncountable
            .clone()
            .into_iter()
            .map(|x| {
                Regex::new(&format!(r"(?i)\b({})\z", x)).unwrap()
            })
            .collect();

        let add_irregular = |
            plurals: &mut Vec<(String, String)>,
            singulars: &mut Vec<(String, String)>,
            singular: String,
            plural: String
        | {
            let singular_first_char = &singular[..1];
            let plural_first_char = &plural[..1];

            let plural_stem = &plural[1..];
            let singular_stem = &singular[1..];

            if singular_first_char.to_uppercase() == plural_first_char.to_uppercase() {
                // let a: &'static str = (r"(?i)(?P<a>".to_owned() + singular_first_char); // + r")" + singular_stem.to_owned() + "$";
                plurals.insert(
                    0,
                    (
                        format!(r"(?i)(?P<a>{}){}$", singular_first_char, singular_stem),
                        format!("{}{}", r"${a}".to_owned(), plural_stem),
                    ),
                );
                plurals.insert(
                    0,
                    (
                        format!(r"(?i)(?P<a>{}){}$", plural_first_char, plural_stem),
                        format!("{}{}", r"${a}".to_owned(), plural_stem),
                    ),
                );

                singulars.insert(
                    0,
                    (
                        format!(r"(?i)(?P<a>{}){}$", plural_first_char, plural_stem),
                        format!("{}{}", r"${a}".to_owned(), singular_stem),
                    ),
                );
            } else {
                let plural_copy_upper1 =
                    format!("{}{}", plural_first_char.to_uppercase(), plural_stem);

                let plural_copy_lower1 =
                    format!("{}{}", plural_first_char.to_lowercase(), plural_stem);

                let plural_copy_upper2 =
                    format!("{}{}", plural_first_char.to_uppercase(), plural_stem);

                let plural_copy_lower2 =
                    format!("{}{}", plural_first_char.to_lowercase(), plural_stem);

                let singular_copy_upper1 =
                    format!("{}{}", singular_first_char.to_uppercase(), singular_stem);

                let singular_copy_lower1 =
                    format!("{}{}", singular_first_char.to_lowercase(), singular_stem);

                plurals.insert(
                    0,
                    (
                        format!(
                            r"{}{}$",
                            singular_first_char.to_uppercase(),
                            case_insensitive!(singular_stem)
                        ),
                        plural_copy_upper1,
                    ),
                );
                plurals.insert(
                    0,
                    (
                        format!(
                            r"{}{}$",
                            singular_first_char.to_lowercase(),
                            case_insensitive!(singular_stem)
                        ),
                        plural_copy_lower1,
                    ),
                );
                plurals.insert(
                    0,
                    (
                        format!(
                            r"{}{}$",
                            plural_first_char.to_uppercase(),
                            case_insensitive!(plural_stem)
                        ),
                        plural_copy_upper2,
                    ),
                );
                plurals.insert(
                    0,
                    (
                        format!(
                            r"{}{}$",
                            plural_first_char.to_lowercase(),
                            case_insensitive!(plural_stem)
                        ),
                        plural_copy_lower2,
                    ),
                );
                singulars.insert(
                    0,
                    (
                        format!(
                            r"{}{}$",
                            plural_first_char.to_uppercase(),
                            case_insensitive!(plural_stem)
                        ),
                        singular_copy_upper1,
                    ),
                );
                singulars.insert(
                    0,
                    (
                        format!(
                            r"{}{}$",
                            plural_first_char.to_lowercase(),
                            case_insensitive!(plural_stem)
                        ),
                        singular_copy_lower1,
                    ),
                );
            }
        };

        add_irregular(&mut plurals, &mut singulars, "person".to_string(), "people".to_string());
        add_irregular(&mut plurals, &mut singulars, "man".to_string(), "men".to_string());
        add_irregular(&mut plurals, &mut singulars, "human".to_string(), "humans".to_string());
        add_irregular(&mut plurals, &mut singulars, "child".to_string(), "children".to_string());
        add_irregular(&mut plurals, &mut singulars, "sex".to_string(), "sexes".to_string());
        add_irregular(&mut plurals, &mut singulars, "move".to_string(), "moves".to_string());
        add_irregular(&mut plurals, &mut singulars, "cow".to_string(), "kine".to_string());
        add_irregular(&mut plurals, &mut singulars, "zombie".to_string(), "zombies".to_string());
        add_irregular(&mut plurals, &mut singulars, "slave".to_string(), "slaves".to_string());
        add_irregular(&mut plurals, &mut singulars, "this".to_string(), "this".to_string());
        add_irregular(&mut plurals, &mut singulars, "flour".to_string(), "flour".to_string());
        add_irregular(&mut plurals, &mut singulars, "milk".to_string(), "milk".to_string());
        add_irregular(&mut plurals, &mut singulars, "water".to_string(), "water".to_string());
        add_irregular(&mut plurals, &mut singulars, "reserve".to_string(), "reserves".to_string());
        add_irregular(&mut plurals, &mut singulars, "gas".to_string(), "gasses".to_string());
        add_irregular(&mut plurals, &mut singulars, "bias".to_string(), "biases".to_string());
        add_irregular(&mut plurals, &mut singulars, "atlas".to_string(), "atlases".to_string());
        add_irregular(&mut plurals, &mut singulars, "goose".to_string(), "geese".to_string());
        add_irregular(&mut plurals, &mut singulars, "pasta".to_string(), "pastas".to_string());
        add_irregular(&mut plurals, &mut singulars, "slice".to_string(), "slices".to_string());
        add_irregular(&mut plurals, &mut singulars, "cactus".to_string(), "cacti".to_string());
        add_irregular(&mut plurals, &mut singulars, "buzz".to_string(), "buzzes".to_string());
        add_irregular(&mut plurals, &mut singulars, "spectrum".to_string(), "spectra".to_string());
        add_irregular(&mut plurals, &mut singulars, "taxon".to_string(), "taxa".to_string());

        let plurals: Vec<(Regex, String)> = plurals
            .into_iter()
            .map(|(rule, repl)| {
                (Regex::new(&rule).unwrap(), repl)
            })
            .collect();
        
        let singulars: Vec<(Regex, String)> = singulars
            .into_iter()
            .map(|(rule, repl)| {
                (Regex::new(&rule).unwrap(), repl)
            })
            .collect();

        (uncountable, plurals, singulars, uncountable_progs)
    };
}

#[doc = include_str ! ("./../README.md")]
pub mod inflection {
    use std::collections::HashSet;
    use regex::Regex;
    use lazy_static::lazy_static;

    use crate::UPS;

    #[inline]
    fn get_uncountable() -> &'static HashSet<String> {
        &UPS.0
    }

    #[inline]
    fn get_uncountable_compiled() -> &'static Vec<Regex> {
        &UPS.3
    }

    #[inline]
    fn get_plurals() -> &'static Vec<(Regex, String)> {
        &UPS.1
    }

    #[inline]
    fn get_singulars() -> &'static Vec<(Regex, String)> {
        &UPS.2
    }

    macro_rules! create_ordinal_function {
        ($func_name:ident, $abs:expr, $param_type:ty) => {
            pub fn $func_name(number: $param_type) -> String {
                let n = $abs(number);
                match n % 100 {
                    11 | 12 | 13 => "th".to_string(),
                    _ => match n % 10 {
                        1 => "st".to_string(),
                        2 => "nd".to_string(),
                        3 => "rd".to_string(),
                        _ => "th".to_string(),
                    },
                }
            }
        };
    }

    macro_rules! create_ordinalize_function {
        ($func_name:ident, $ordinal_function:ident, $param_type:ty) => {
            pub fn $func_name(number: $param_type) -> String {
                format!("{}{}", number, $ordinal_function(number))
            }
        };
    }

    create_ordinal_function!(ordinal_i8, |x: i8| x.abs(), i8);
    create_ordinal_function!(ordinal_i16, |x: i16| x.abs(), i16);
    create_ordinal_function!(ordinal_i32, |x: i32| x.abs(), i32);
    create_ordinal_function!(ordinal_i64, |x: i64| x.abs(), i64);
    create_ordinal_function!(ordinal_i128, |x: i128| x.abs(), i128);
    create_ordinal_function!(ordinal_u8, |x: u8| x, u8);
    create_ordinal_function!(ordinal_u16, |x: u16| x, u16);
    create_ordinal_function!(ordinal_u32, |x: u32| x, u32);
    create_ordinal_function!(ordinal_u64, |x: u64| x, u64);
    create_ordinal_function!(ordinal_u128, |x: u128| x, u128);
    create_ordinal_function!(ordinal_usize, |x: usize| x, usize);

    create_ordinalize_function!(ordinalize_i8, ordinal_i8, i8);
    create_ordinalize_function!(ordinalize_i16, ordinal_i16, i16);
    create_ordinalize_function!(ordinalize_i32, ordinal_i32, i32);
    create_ordinalize_function!(ordinalize_i64, ordinal_i64, i64);
    create_ordinalize_function!(ordinalize_i128, ordinal_i128, i128);
    create_ordinalize_function!(ordinalize_u8, ordinal_u8, u8);
    create_ordinalize_function!(ordinalize_u16, ordinal_u16, u16);
    create_ordinalize_function!(ordinalize_u32, ordinal_u32, u32);
    create_ordinalize_function!(ordinalize_u64, ordinal_u64, u64);
    create_ordinalize_function!(ordinalize_u128, ordinal_u128, u128);
    create_ordinalize_function!(ordinalize_usize, ordinal_usize, usize);

    pub fn camelize<S: AsRef<str>>(string: S) -> String {
        camelize_upper(string, true)
    }

    pub fn camelize_upper<S: AsRef<str>>(string: S, uppercase_first_letter: bool) -> String {
        let input_string = string.as_ref().to_owned();

        if input_string.is_empty() {
            return input_string;
        }

        if uppercase_first_letter {
            lazy_static! {
                static ref CU_RE: Regex = Regex::new(r"(?:^|_)(.)").unwrap();
            }
            let mut result: String = input_string.to_owned();

            for cap in CU_RE.find_iter(&input_string) {
                let replace_with = &cap
                    .as_str()
                    .chars()
                    .last()
                    .unwrap_or(' ')
                    .to_uppercase()
                    .to_string();
                result.replace_range(cap.range(), replace_with);
            }
            return result;
        }

        let input_string = camelize_upper(input_string, true);
        let mut result = string
            .as_ref()
            .to_string()
            .chars()
            .next()
            .unwrap_or(' ')
            .to_lowercase()
            .to_string();
        result.push_str(&input_string[1..]);
        result
    }

    pub fn dasherize<S: AsRef<str>>(word: S) -> String {
        word.as_ref().to_string().replace('_', "-")
    }

    pub fn humanize<S: AsRef<str>>(word: S) -> String {
        lazy_static! {
            static ref H_ID_PROG: Regex = Regex::new(r"_id$").unwrap();
            static ref H_STEM_PROG: Regex = Regex::new(r"(?i)([a-z\d]*)").unwrap();
            static ref H_WORD_PROG: Regex = Regex::new(r"^\w").unwrap();
        }

        let mut result: String = H_ID_PROG.replace_all(word.as_ref(), "").to_string();
        result = result.replace('_', " ");

        if result.is_empty() {
            return result;
        }

        let updated_result = result.to_owned();
        for cap in H_STEM_PROG.find_iter(&updated_result) {
            let replace_with = cap.as_str().to_lowercase().to_string();
            result.replace_range(cap.range(), &replace_with);
        }

        let updated_result = result.to_owned();
        for cap in H_WORD_PROG.find_iter(&updated_result) {
            let mut replace_with = cap
                .as_str()
                .chars()
                .next()
                .unwrap_or(' ')
                .to_uppercase()
                .to_string();
            let last_part = &cap.as_str()[1..];
            replace_with.push_str(last_part);
            result.replace_range(cap.range(), &replace_with);
        }
        result
    }

    pub fn underscore<S: AsRef<str>>(string: S) -> String {
        lazy_static! {
            static ref U_PROG1: Regex = Regex::new(r"(?P<a>[A-Z]+)(?P<b>[A-Z][a-z])").unwrap();
            static ref U_PROG2: Regex = Regex::new(r"(?P<a>[a-z\d])(?P<b>[A-Z])").unwrap();
        }
        let stand_in = "$a-$b";
        let mut word = string.as_ref().to_string();
        word = U_PROG1.replace_all(&word, stand_in).to_string();
        word = U_PROG2.replace_all(&word, stand_in).to_string();
        word = word.replace('-', "_");
        word.to_lowercase()
    }

    pub fn transliterate<S: AsRef<str>>(string: S) -> String {
        deunicode::deunicode(string.as_ref())
    }

    pub fn parameterize_with_sep<S: AsRef<str>>(string: S, sep: String) -> String {
        let transliterated0 = transliterate(string);
        let transliterated = transliterated0.as_str();

        let is_sep_empty = sep.is_empty();
        let sep_copy = sep.to_owned();

        lazy_static! {
            static ref PWS_CLEAN_PROG: Regex = Regex::new(r"(?i)[^a-z0-9\-_]+").unwrap();
        }
        let cleaned0 = PWS_CLEAN_PROG.replace_all(transliterated, sep);
        let cleaned = cleaned0.as_ref();
        if !is_sep_empty {
            let re_sep = regex::escape(&sep_copy);
            let sep_prog = Regex::new(&format!(r"{}{}", re_sep, re_sep)).unwrap();
            let leading_sep_prog = Regex::new(&format!(r"(?i)^{}|{}$", re_sep, re_sep)).unwrap();

            let rm_sep = sep_prog.replace_all(cleaned, sep_copy);
            let rm_sep = leading_sep_prog.replace_all(&rm_sep, "");

            return rm_sep.as_ref().to_lowercase();
        }

        cleaned.to_lowercase()
    }

    pub fn parameterize<S: AsRef<str>>(string: S) -> String {
        parameterize_with_sep::<S>(string, "-".to_string())
    }

    pub fn pluralize<S: AsRef<str>>(string: S) -> String {
        let word: &str = string.as_ref();
        let word_is_empty = word.is_empty();
        let word_is_in_uncountable: bool =
            get_uncountable().contains(word.to_lowercase().as_str());

        if word_is_empty || word_is_in_uncountable {
            return word.to_string();
        }

        for (rule, repl) in get_plurals().iter() {
            // let re = Regex::new(rule).unwrap();
            if rule.is_match(word) {
                return rule.replace_all(word, repl).to_string();
            }
        }

        word.to_string()
    }

    pub fn singularize<S: AsRef<str>>(string: S) -> String {
        let word = string.as_ref();

        for re in get_uncountable_compiled().iter() {
            // let pattern = &format!(r"(?i)\b({})\z", inf);
            // let re = Regex::new(pattern).unwrap();
            if re.is_match(word) {
                return word.to_string();
            }
        }

        for (rule, repl) in get_singulars().iter() {
            // let re = Regex::new(rule).unwrap();
            if rule.is_match(word) {
                return rule.replace_all(word, repl).to_string();
            }
        }

        word.to_string()
    }

    pub fn tableize<S: AsRef<str>>(string: S) -> String {
        let underscore = underscore(string);
        pluralize(underscore)
    }

    fn capitalize<S: AsRef<str>>(s: S) -> String {
        let mut c = s.as_ref().chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().chain(c).collect(),
        }
    }

    pub fn titleize<S: AsRef<str>>(string: S) -> String {
        let input_string = string.as_ref();
        let mut result: String = underscore(&string);
        result = humanize(result);
        lazy_static! {
            static ref H_FIRST_PROG: Regex = Regex::new(r"\b((\s+)?'?\w)").unwrap();
        }
        for cap in H_FIRST_PROG.find_iter(input_string) {
            result.replace_range(cap.range(), cap.as_str());
        }
        result = result
            .split(char::is_whitespace)
            .map(|word| format!(" {}", capitalize(word)))
            .collect::<String>()
            .trim()
            .to_string();
        result
    }

    pub fn normalize_spaces<S: AsRef<str>>(string: S) -> String {
        lazy_static! {
            static ref NS_RE: Regex = Regex::new(r"\s+").unwrap();
        }
        let text = string.as_ref();
        return NS_RE.replace_all(text, " ").trim().to_string();
    }

    fn _only_alpha<S: AsRef<str>>(
        string: S,
        check_fn: fn(c: &char) -> bool,
        repl: Option<char>,
    ) -> String {
        let chars = string.as_ref().chars();

        chars
            .filter_map(|c| if !check_fn(&c) { repl } else { Some(c) })
            .collect()
    }

    pub fn only_alpha<S: AsRef<str>>(string: S, repl: Option<char>) -> String {
        let check_fn = |c: &char| c.is_alphabetic();
        _only_alpha(string.as_ref(), check_fn, repl)
    }

    pub fn only_alphanum<S: AsRef<str>>(string: S, repl: Option<char>) -> String {
        let check_fn = |c: &char| c.is_alphanumeric();
        _only_alpha(string.as_ref(), check_fn, repl)
    }

    pub fn only_alpha_ascii<S: AsRef<str>>(string: S, repl: Option<char>) -> String {
        let check_fn = |c: &char| c.is_ascii_alphabetic();
        _only_alpha(string.as_ref(), check_fn, repl)
    }

    pub fn only_alphanum_ascii<S: AsRef<str>>(string: S, repl: Option<char>) -> String {
        let check_fn = |c: &char| c.is_ascii_alphanumeric();
        _only_alpha(string.as_ref(), check_fn, repl)
    }

    pub fn keyify<S: AsRef<str>>(string: S) -> String {
        let result = only_alphanum_ascii(string, Some(' '));
        let result = normalize_spaces(result);
        let result = titleize(result);
        let result = parameterize_with_sep(result, "_".to_string());
        underscore(result.trim())
    }
}

#[cfg(test)]
mod tests {
    use crate::inflection;

    const SINGULAR_TO_PLURAL: [(&str, &str); 90] = [
        ("search", "searches"),
        ("switch", "switches"),
        ("fix", "fixes"),
        ("box", "boxes"),
        ("process", "processes"),
        ("address", "addresses"),
        ("case", "cases"),
        ("stack", "stacks"),
        ("wish", "wishes"),
        ("fish", "fish"),
        ("jeans", "jeans"),
        ("funky jeans", "funky jeans"),
        ("category", "categories"),
        ("query", "queries"),
        ("ability", "abilities"),
        ("agency", "agencies"),
        ("movie", "movies"),
        ("archive", "archives"),
        ("index", "indices"),
        ("wife", "wives"),
        ("safe", "saves"),
        ("half", "halves"),
        ("move", "moves"),
        ("salesperson", "salespeople"),
        ("person", "people"),
        ("spokesman", "spokesmen"),
        ("man", "men"),
        ("woman", "women"),
        ("basis", "bases"),
        ("diagnosis", "diagnoses"),
        ("diagnosis_a", "diagnosis_as"),
        ("datum", "data"),
        ("medium", "media"),
        ("stadium", "stadia"),
        ("analysis", "analyses"),
        ("node_child", "node_children"),
        ("child", "children"),
        ("experience", "experiences"),
        ("day", "days"),
        ("comment", "comments"),
        ("foobar", "foobars"),
        ("newsletter", "newsletters"),
        ("old_news", "old_news"),
        ("news", "news"),
        ("series", "series"),
        ("species", "species"),
        ("quiz", "quizzes"),
        ("perspective", "perspectives"),
        ("ox", "oxen"),
        ("passerby", "passersby"),
        ("photo", "photos"),
        ("buffalo", "buffaloes"),
        ("tomato", "tomatoes"),
        ("potato", "potatoes"),
        ("dwarf", "dwarves"),
        ("elf", "elves"),
        ("information", "information"),
        ("equipment", "equipment"),
        ("bus", "buses"),
        ("status", "statuses"),
        ("status_code", "status_codes"),
        ("mouse", "mice"),
        ("louse", "lice"),
        ("house", "houses"),
        ("octopus", "octopi"),
        ("virus", "viri"),
        ("alias", "aliases"),
        ("portfolio", "portfolios"),
        ("vertex", "vertices"),
        ("matrix", "matrices"),
        ("matrix_fu", "matrix_fus"),
        ("axis", "axes"),
        ("testis", "testes"),
        ("crisis", "crises"),
        ("rice", "rice"),
        ("shoe", "shoes"),
        ("horse", "horses"),
        ("prize", "prizes"),
        ("edge", "edges"),
        ("cow", "kine"),
        ("database", "databases"),
        ("human", "humans"),
        ("flour", "flour"),
        ("water", "water"),
        ("slave", "slaves"),
        ("milk", "milk"),
        ("reserve", "reserves"),
        ("gas", "gasses"),
        ("bias", "biases"),
        ("atlas", "atlases"),
    ];

    const CAMEL_TO_UNDERSCORE: [(&str, &str); 4] = [
        ("Product", "product"),
        ("SpecialGuest", "special_guest"),
        ("ApplicationController", "application_controller"),
        ("Area51Controller", "area51_controller"),
    ];

    const CAMEL_TO_UNDERSCORE_WITHOUT_REVERSE: [(&str, &str); 4] = [
        ("HTMLTidy", "html_tidy"),
        ("HTMLTidyGenerator", "html_tidy_generator"),
        ("FreeBSD", "free_bsd"),
        ("HTML", "html"),
    ];

    const STRING_TO_PARAMETERIZED: [(&str, &str); 8] = [
        (r"Donald E. Knuth", "donald-e-knuth"),
        (
            r"Random text with *(bad)* characters",
            "random-text-with-bad-characters",
        ),
        (r"Allow_Under_Scores", "allow_under_scores"),
        (r"Trailing bad characters!@#", "trailing-bad-characters"),
        (r"!@#Leading bad characters", "leading-bad-characters"),
        (r"Squeeze   separators", "squeeze-separators"),
        (r"Test with + sign", "test-with-sign"),
        (
            r"Test with malformed utf8 \251",
            "test-with-malformed-utf8-251",
        ),
    ];

    const STRING_TO_PARAMETERIZE_WITH_NO_SEPARATOR: [(&str, &str); 8] = [
        (r"Donald E. Knuth", "donaldeknuth"),
        (r"With-some-dashes", "with-some-dashes"),
        (
            r"Random text with *(bad)* characters",
            "randomtextwithbadcharacters",
        ),
        (r"Trailing bad characters!@#", "trailingbadcharacters"),
        (r"!@#Leading bad characters", "leadingbadcharacters"),
        (r"Squeeze   separators", "squeezeseparators"),
        (r"Test with + sign", "testwithsign"),
        (r"Test with malformed utf8 \251", "testwithmalformedutf8251"),
    ];

    const STRING_TO_PARAMETERIZE_WITH_UNDERSCORE: [(&str, &str); 9] = [
        (r"Donald E. Knuth", "donald_e_knuth"),
        (
            r"Random text with *(bad)* characters",
            "random_text_with_bad_characters",
        ),
        (r"With-some-dashes", "with-some-dashes"),
        (r"Retain_underscore", "retain_underscore"),
        (r"Trailing bad characters!@#", "trailing_bad_characters"),
        (r"!@#Leading bad characters", "leading_bad_characters"),
        (r"Squeeze   separators", "squeeze_separators"),
        (r"Test with + sign", "test_with_sign"),
        (
            r"Test with malformed utf8 \251",
            "test_with_malformed_utf8_251",
        ),
    ];

    const KEYIFY_BULK: [(&str, &str); 18] = [
        (r"Donald E. Knuth", "donald_e_knuth"),
        (
            r"Random text with *(bad)* characters",
            "random_text_with_bad_characters",
        ),
        (r"With-some-dashes", "with_some_dashes"),
        (r"Retain_underscore", "retain_underscore"),
        (r"Trailing bad characters!@#", "trailing_bad_characters"),
        (r"!@#Leading bad characters", "leading_bad_characters"),
        (r"Squeeze   separators", "squeeze_separators"),
        (r"Test with + sign", "test_with_sign"),
        (
            r"Test with malformed utf8 \251",
            "test_with_malformed_utf8_251",
        ),
        ("  --== some strange_key", "some_strange_key"),
        ("  --== some otherKey_", "some_other_key"),
        ("  --== some other-key_", "some_other_key"),
        ("Some Other Key", "some_other_key"),
        ("Some-Other-Key", "some_other_key"),
        ("some_other_key", "some_other_key"),
        ("      ", ""),
        (" -----", ""),
        ("========", ""),
    ];

    const STRING_TO_PARAMETERIZED_AND_NORMALIZED: [(&str, &str); 6] = [
        (r"Malmö", "malmo"),
        (r"Garçons", "garcons"),
        (r"Ops\331", "ops-331"),
        (r"Ærøskøbing", "aeroskobing"),
        (r"Aßlar", "asslar"),
        (r"日本語", "ri-ben-yu"),
    ];

    const UNDERSCORE_TO_HUMAN: [(&str, &str); 3] = [
        ("employee_salary", "Employee salary"),
        ("employee_id", "Employee"),
        ("underground", "Underground"),
    ];

    const MIXTURE_TO_TITLEIZED: [(&str, &str); 12] = [
        ("active_record", "Active Record"),
        ("ActiveRecord", "Active Record"),
        ("action web service", "Action Web Service"),
        ("Action Web Service", "Action Web Service"),
        ("Action web service", "Action Web Service"),
        ("actionwebservice", "Actionwebservice"),
        ("Actionwebservice", "Actionwebservice"),
        ("david's code", "David's Code"),
        ("David's code", "David's Code"),
        ("david's Code", "David's Code"),
        ("ana índia", "Ana Índia"),
        ("Ana Índia", "Ana Índia"),
    ];

    const UNDERSCORES_TO_DASHES: [(&str, &str); 3] = [
        ("street", "street"),
        ("street_address", "street-address"),
        ("person_street_address", "person-street-address"),
    ];

    const STRING_TO_TABLEIZE: [(&str, &str); 4] = [
        ("person", "people"),
        ("Country", "countries"),
        ("ChildToy", "child_toys"),
        ("_RecipeIngredient", "_recipe_ingredients"),
    ];

    #[test]
    fn substring() {
        assert_eq!(&"1Hello"[1..], "Hello");
    }

    #[test]
    fn camelize_bulk() {
        for (expected, input) in CAMEL_TO_UNDERSCORE {
            assert_eq!(inflection::camelize(input), expected);
        }
    }

    #[test]
    fn pluralize_bulk() {
        for (input, expected) in SINGULAR_TO_PLURAL {
            assert_eq!(inflection::pluralize(input), expected);
        }
    }

    #[test]
    fn singularize_bulk() {
        for (expected, input) in SINGULAR_TO_PLURAL {
            assert_eq!(inflection::singularize(input), expected);
        }
    }

    #[test]
    fn underscore_bulk() {
        for (expected, input) in UNDERSCORES_TO_DASHES {
            assert_eq!(inflection::underscore(input), expected);
        }

        for (input, expected) in CAMEL_TO_UNDERSCORE_WITHOUT_REVERSE {
            assert_eq!(inflection::underscore(input), expected);
        }
    }

    #[test]
    fn dasherize_bulk() {
        for (input, expected) in UNDERSCORES_TO_DASHES {
            assert_eq!(inflection::dasherize(input), expected);
        }
    }

    #[test]
    fn tableize_bulk() {
        for (input, expected) in STRING_TO_TABLEIZE {
            assert_eq!(inflection::tableize(input), expected);
        }
    }

    #[test]
    fn humanize_bulk() {
        for (input, expected) in UNDERSCORE_TO_HUMAN {
            assert_eq!(inflection::humanize(input), expected);
        }
    }

    #[test]
    fn titleize_bulk() {
        for (input, expected) in MIXTURE_TO_TITLEIZED {
            assert_eq!(inflection::titleize(input), expected);
        }
    }

    #[test]
    fn keyify_test() {
        for (input, expected) in KEYIFY_BULK {
            assert_eq!(inflection::keyify(input), expected);
        }
    }

    #[test]
    fn parameterize_bulk() {
        for (input, expected) in STRING_TO_PARAMETERIZED {
            assert_eq!(inflection::parameterize(input), expected);
        }

        for (input, expected) in STRING_TO_PARAMETERIZED_AND_NORMALIZED {
            assert_eq!(inflection::parameterize(input), expected);
        }

        for (input, expected) in STRING_TO_PARAMETERIZE_WITH_UNDERSCORE {
            assert_eq!(
                inflection::parameterize_with_sep(input, "_".to_string()),
                expected
            );
        }

        for (input, expected) in STRING_TO_PARAMETERIZE_WITH_NO_SEPARATOR {
            assert_eq!(
                inflection::parameterize_with_sep(input, "".to_string()),
                expected
            );
        }
    }

    macro_rules! test_ordinal {
        ($ordinal:ident, $ordinalize:ident, $ordinalize_bulk:ident, $param_type:ty) => {
            #[test]
            fn $ordinal() {
                assert_eq!(inflection::$ordinal(1), "st");
                assert_eq!(inflection::$ordinal(2), "nd");
                assert_eq!(inflection::$ordinal(3), "rd");
                assert_eq!(inflection::$ordinal(4), "th");
                assert_eq!(inflection::$ordinal(10), "th");

                assert_eq!(inflection::$ordinal(1002), "nd");
                assert_eq!(inflection::$ordinal(1003), "rd");
            }

            #[test]
            fn $ordinalize() {
                assert_eq!(inflection::$ordinalize(1), "1st");
                assert_eq!(inflection::$ordinalize(2), "2nd");
                assert_eq!(inflection::$ordinalize(3), "3rd");
                assert_eq!(inflection::$ordinalize(4), "4th");
                assert_eq!(inflection::$ordinalize(10), "10th");
                assert_eq!(inflection::$ordinalize(1002), "1002nd");
                assert_eq!(inflection::$ordinalize(1003), "1003rd");
            }

            #[test]
            fn $ordinalize_bulk() {
                let ordinal_numbers: [($param_type, &str); 31] = [
                    (0, "0th"),
                    (1, "1st"),
                    (2, "2nd"),
                    (3, "3rd"),
                    (4, "4th"),
                    (5, "5th"),
                    (6, "6th"),
                    (7, "7th"),
                    (8, "8th"),
                    (9, "9th"),
                    (10, "10th"),
                    (11, "11th"),
                    (12, "12th"),
                    (13, "13th"),
                    (14, "14th"),
                    (20, "20th"),
                    (21, "21st"),
                    (22, "22nd"),
                    (23, "23rd"),
                    (24, "24th"),
                    (100, "100th"),
                    (101, "101st"),
                    (102, "102nd"),
                    (103, "103rd"),
                    (104, "104th"),
                    (110, "110th"),
                    (111, "111th"),
                    (112, "112th"),
                    (113, "113th"),
                    (1000, "1000th"),
                    (1001, "1001st"),
                ];

                for (input, expected) in ordinal_numbers {
                    assert_eq!(inflection::$ordinalize(input), expected);
                }
            }
        };
    }

    test_ordinal!(ordinal_u16, ordinalize_u16, oridinalize_u16_bulk, u16);
    test_ordinal!(ordinal_u32, ordinalize_u32, oridinalize_u32_bulk, u32);
    test_ordinal!(ordinal_u64, ordinalize_u64, oridinalize_u64_bulk, u64);
    test_ordinal!(ordinal_u128, ordinalize_u128, oridinalize_u128_bulk, u128);

    test_ordinal!(ordinal_i16, ordinalize_i16, oridinalize_i16_bulk, i16);
    test_ordinal!(ordinal_i32, ordinalize_i32, oridinalize_i32_bulk, i32);
    test_ordinal!(ordinal_i64, ordinalize_i64, oridinalize_i64_bulk, i64);
    test_ordinal!(ordinal_i128, ordinalize_i128, oridinalize_i128_bulk, i128);
}
