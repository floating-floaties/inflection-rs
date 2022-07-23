#[allow(dead_code)]

mod inflection {
    use std::collections::{HashMap, HashSet};

    use deunicode::deunicode;
    use regex::Regex;

    macro_rules! substr {
        ($str:expr, $start_pos:expr) => {{
            substr!($str, $start_pos, $str.len())
        }};

        ($str:expr, $start_pos:expr, $end_pos:expr) => {{
            // TODO: may overflow
            substr!($str, $start_pos, $end_pos - $start_pos, true)
        }};

        ($str:expr, $start_pos:expr, $take_count:expr, $use_take:expr) => {{
            &$str
                .chars()
                .skip($start_pos)
                .take($take_count)
                .collect::<String>()
        }};
    }

    macro_rules! case_insensitive {
        ($str:expr) => {{
            &$str
                .as_str()
                .chars()
                .map(|c| format!("[{}{}]", c.to_string(), c.to_uppercase().to_string()))
                .collect::<String>()
        }};
    }
    pub(crate) use substr;

    fn example(a: String, start: usize, end: usize) -> String {
        substr!(a, start, end).to_string()
    }

    pub struct Inflection {
        plurals: Vec<(String, String)>,
        singulars: Vec<(String, String)>,
        uncountable: HashSet<String>,
        regex_cache: HashMap<String, Regex>,
    }

    impl Inflection {
        fn init() -> Self {
            let plurals: Vec<(String, String)> = vec![
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
                    r"(?i)(?P<a>buffal|potat|tomat)o$".to_string(),
                    "${a}oes".to_string(),
                ),
                (r"(?i)(?P<a>bu)s$".to_string(), "${a}ses".to_string()),
                (
                    r"(?i)(?P<a>alias|status)$".to_string(),
                    "${a}es".to_string(),
                ),
                (r"(?i)(?P<a>octop|vir)i$".to_string(), "${a}i".to_string()),
                (r"(?i)(?P<a>octop|vir)us$".to_string(), "${a}i".to_string()),
                (r"(?i)^(?P<a>ax|test)is$".to_string(), "${a}es".to_string()),
                (r"(?i)s$".to_string(), "s".to_string()),
                (r"$".to_string(), "s".to_string()),
            ];

            let singulars: Vec<(String, String)> = vec![
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
                    r"(?i)(?P<a>octop|vir)(us|i)$".to_string(),
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

            let uncountable: HashSet<String> = HashSet::from([
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

            return Inflection {
                singulars,
                plurals,
                uncountable,
                regex_cache: HashMap::new(),
            };
        }

        pub fn new() -> Self {
            let mut result = Self::init();
            result.irregular("person".to_string(), "people".to_string());
            result.irregular("man".to_string(), "men".to_string());
            result.irregular("human".to_string(), "humans".to_string());
            result.irregular("child".to_string(), "children".to_string());
            result.irregular("sex".to_string(), "sexes".to_string());
            result.irregular("move".to_string(), "moves".to_string());
            result.irregular("cow".to_string(), "kine".to_string());
            result.irregular("zombie".to_string(), "zombies".to_string());

            return result;
        }

        fn compile_regex<S: AsRef<str>>(&mut self, pattern: S) -> Regex {
            let expression = pattern.as_ref().to_string();
            match self.regex_cache.get(&expression) {
                Some(re) => re.to_owned(),
                _ => {
                    let re = Regex::new(&expression).unwrap();
                    self.regex_cache.insert(expression, re.to_owned());
                    return re;
                }
            }
        }

        fn irregular(&mut self, singular: String, plural: String) {
            let singular_first_char: char = singular.chars().nth(0).unwrap();
            let plural_first_char: char = plural.chars().nth(0).unwrap(); //.collect::<Vec<char>>()[0].to_string();

            let plural_stem = substr!(plural, 1);
            let singular_stem = substr!(singular, 1);

            if singular_first_char.to_string().to_uppercase()
                == plural_first_char.to_string().to_uppercase()
            {
                self.plurals.insert(
                    0,
                    (
                        format!(r"(?i)(?P<a>{}){}$", singular_first_char, singular_stem),
                        format!("{}{}", r"${a}".to_owned(), plural_stem),
                    ),
                );
                self.plurals.insert(
                    0,
                    (
                        format!(r"(?i)(?P<a>{}){}$", plural_first_char, plural_stem),
                        format!("{}{}", r"${a}".to_owned(), plural_stem),
                    ),
                );

                self.singulars.insert(
                    0,
                    (
                        format!(r"(?i)(?P<a>{}){}$", plural_first_char, plural_stem),
                        format!("{}{}", r"${a}".to_owned(), singular_stem),
                    ),
                );
            } else {
                let plural_copy_upper1 = format!(
                    "{}{}",
                    plural_first_char.to_uppercase().to_string(),
                    plural_stem
                );

                let plural_copy_lower1 = format!(
                    "{}{}",
                    plural_first_char.to_lowercase().to_string(),
                    plural_stem
                );

                let plural_copy_upper2 = format!(
                    "{}{}",
                    plural_first_char.to_uppercase().to_string(),
                    plural_stem
                );

                let plural_copy_lower2 = format!(
                    "{}{}",
                    plural_first_char.to_lowercase().to_string(),
                    plural_stem
                );

                let singular_copy_upper1 = format!(
                    "{}{}",
                    singular_first_char.to_uppercase().to_string(),
                    singular_stem
                );

                let singular_copy_lower1 = format!(
                    "{}{}",
                    singular_first_char.to_lowercase().to_string(),
                    singular_stem
                );

                self.plurals.insert(
                    0,
                    (
                        format!(
                            r"{}{}$",
                            singular_first_char.to_uppercase().to_string(),
                            case_insensitive!(singular_stem)
                        ),
                        plural_copy_upper1,
                    ),
                );
                self.plurals.insert(
                    0,
                    (
                        format!(
                            r"{}{}$",
                            singular_first_char.to_lowercase().to_string(),
                            case_insensitive!(singular_stem)
                        ),
                        plural_copy_lower1,
                    ),
                );
                self.plurals.insert(
                    0,
                    (
                        format!(
                            r"{}{}$",
                            plural_first_char.to_uppercase().to_string(),
                            case_insensitive!(plural_stem)
                        ),
                        plural_copy_upper2,
                    ),
                );
                self.plurals.insert(
                    0,
                    (
                        format!(
                            r"{}{}$",
                            plural_first_char.to_lowercase().to_string(),
                            case_insensitive!(plural_stem)
                        ),
                        plural_copy_lower2,
                    ),
                );
                self.singulars.insert(
                    0,
                    (
                        format!(
                            r"{}{}$",
                            plural_first_char.to_uppercase().to_string(),
                            case_insensitive!(plural_stem)
                        ),
                        singular_copy_upper1,
                    ),
                );
                self.singulars.insert(
                    0,
                    (
                        format!(
                            r"{}{}$",
                            plural_first_char.to_lowercase().to_string(),
                            case_insensitive!(plural_stem)
                        ),
                        singular_copy_lower1,
                    ),
                );
            }
        }

        pub fn camelize<S: AsRef<str>>(&mut self, string: S) -> String {
            return self.camelize_upper(string, true);
        }

        pub fn camelize_upper<S: AsRef<str>>(
            &mut self,
            string: S,
            uppercase_first_letter: bool,
        ) -> String {
            let input_string = string.as_ref().to_owned();

            if uppercase_first_letter {
                let re = self.compile_regex(r"(?:^|_)(.)");
                let mut result: String = input_string.to_owned();

                for cap in re.find_iter(&input_string) {
                    let replace_with = &cap
                        .as_str()
                        .chars()
                        .last()
                        .unwrap()
                        .to_uppercase()
                        .to_string();
                    result.replace_range(cap.range(), replace_with);
                }
                return result;
            }

            let input_string = self.camelize_upper(input_string, true);
            let mut result = string
                .as_ref()
                .to_string()
                .chars()
                .nth(0)
                .unwrap()
                .to_lowercase()
                .to_string();
            result.push_str(substr!(input_string, 1));
            result
        }

        pub fn dasherize<S: AsRef<str>>(&mut self, word: S) -> String {
            word.as_ref().to_string().replace("_", "-")
        }

        pub fn humanize<S: AsRef<str>>(&mut self, word: S) -> String {
            let id_prog = self.compile_regex(r"_id$");
            let stem_prog = self.compile_regex(r"(?i)([a-z\d]*)");
            let word_prog = self.compile_regex(r"^\w");

            let mut result: String = id_prog.replace_all(word.as_ref(), "").to_string();
            result = result.replace("_", " ");

            let updated_result = result.to_owned();
            for cap in stem_prog.find_iter(&updated_result) {
                let replace_with = cap.as_str().to_lowercase().to_string();
                result.replace_range(cap.range(), &replace_with);
            }

            let updated_result = result.to_owned();
            for cap in word_prog.find_iter(&updated_result) {
                let mut replace_with = cap
                    .as_str()
                    .chars()
                    .nth(0)
                    .unwrap()
                    .to_uppercase()
                    .to_string();
                let last_part = substr!(cap.as_str().to_string(), 1);
                replace_with.push_str(last_part);
                result.replace_range(cap.range(), &replace_with);
            }
            result
        }

        pub fn ordinal_unsigned(&self, number: usize) -> String {
            match number % 100 {
                11 | 12 | 13 => "th".to_string(),
                _ => match number % 10 {
                    1 => "st".to_string(),
                    2 => "nd".to_string(),
                    3 => "rd".to_string(),
                    _ => "th".to_string(),
                },
            }
        }

        pub fn ordinal(&self, number: i128) -> String {
            let n = number.abs();
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

        pub fn ordinalize_unsigned(&self, number: usize) -> String {
            format!("{}{}", number, self.ordinal_unsigned(number))
        }

        pub fn ordinalize(&self, number: i128) -> String {
            format!("{}{}", number, self.ordinal(number))
        }

        pub fn underscore<S: AsRef<str>>(&mut self, string: S) -> String {
            let prog1 = self.compile_regex(r"(?P<a>[A-Z]+)(?P<b>[A-Z][a-z])");
            let prog2 = self.compile_regex(r"(?P<a>[a-z\d])(?P<b>[A-Z])");
            let stand_in = "$a-$b";
            let mut word = string.as_ref().to_string();
            word = prog1.replace_all(&word, stand_in).to_string();
            word = prog2.replace_all(&word, stand_in).to_string();
            word = word.replace("-", "_");
            word.to_lowercase()
        }

        pub fn transliterate<S: AsRef<str>>(&self, string: S) -> String {
            deunicode(string.as_ref())
        }

        pub fn parameterize_with_sep<S: AsRef<str>>(&mut self, string: S, sep: String) -> String {
            let mut result = self.transliterate(string);

            let is_sep_empty = sep.is_empty();
            let sep_copy = sep.to_owned();

            let clean_prog = self.compile_regex(r"(?i)[^a-z0-9\-_]+");
            result = clean_prog.replace_all(&result, sep).to_string();

            if !is_sep_empty {
                let re_sep = regex::escape(&sep_copy);
                let sep_prog = self.compile_regex(format!(r"{}{}", re_sep, re_sep));
                let leading_sep_prog = self.compile_regex(format!(r"(?i)^{}|{}$", re_sep, re_sep));
                result = sep_prog.replace_all(&result, sep_copy).to_string();
                result = leading_sep_prog.replace_all(&result, "").to_string();
            }

            result.to_lowercase()
        }

        pub fn parameterize<S: AsRef<str>>(&mut self, string: S) -> String {
            self.parameterize_with_sep::<S>(string, "-".to_string())
        }

        pub fn pluralize<S: AsRef<str>>(&mut self, string: S) -> String {
            let word: String = string.as_ref().to_string();
            let word_is_empty: bool = word.is_empty();
            let word_is_in_uncountable: bool = self.uncountable.contains(&word.to_lowercase());

            // println!("word={word}; word_is_empty={word_is_empty}; word_is_in_uncountables={word_is_in_uncountable}");
            if word_is_empty || word_is_in_uncountable {
                return word;
            }

            for (rule, repl) in self.plurals.iter() {
                // TODO: use cache for regex compilation
                let re = Regex::new(rule).unwrap();
                if re.find(&word).is_some() {
                    return re.replace_all(&word, repl).to_string();
                }
            }

            word
        }

        pub fn singularize<S: AsRef<str>>(&mut self, string: S) -> String {
            let word: String = string.as_ref().to_string();
            for inf in self.uncountable.iter() {
                // TODO: use cache for regex compilation
                let re = Regex::new(format!(r"(?i)\b({})\z", inf).as_ref()).unwrap();
                if re.find(&word).is_some() {
                    return word;
                }
            }

            for (rule, repl) in self.singulars.iter() {
                // TODO: use cache for regex compilation
                let re = Regex::new(rule).unwrap();
                if re.find(&word).is_some() {
                    return re.replace_all(&word, repl).to_string();
                }
            }

            word
        }

        pub fn tableize<S: AsRef<str>>(&mut self, string: S) -> String {
            let underscore = self.underscore(string);
            self.pluralize(underscore)
        }

        fn capitalize<S: AsRef<str>>(&self, s: S) -> String {
            let mut c = s.as_ref().chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(c).collect(),
            }
        }

        pub fn titleize<S: AsRef<str>>(&mut self, string: S) -> String {
            let input_string = string.as_ref().to_owned();
            let mut result: String = self.underscore(string);
            result = self.humanize(result);
            // let result = self.title(result);

            let first_prog = self.compile_regex(r"\b((\s+)?'?\w)");
            for cap in first_prog.find_iter(&input_string) {
                result.replace_range(cap.range(), cap.as_str());
            }
            result = result
                .split(char::is_whitespace)
                .map(|word| format!(" {}", self.capitalize(word)))
                .collect::<String>()
                .trim()
                .to_string();

            result
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::inflection::{substr, Inflection};

    const SINGULAR_TO_PLURAL: [(&str, &str); 82] = [
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
        (r"Test with malformed utf8 \251", "test-with-malformed-utf8"),
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
        (r"Test with malformed utf8 \251", "testwithmalformedutf8"),
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
        (r"Test with malformed utf8 \251", "test_with_malformed_utf8"),
    ];

    const STRING_TO_PARAMETERIZED_AND_NORMALIZED: [(&str, &str); 6] = [
        (r"Malmö", "malmo"),
        (r"Garçons", "garcons"),
        (r"Ops\331", "opsu"),
        (r"Ærøskøbing", "rskbing"),
        (r"Aßlar", "alar"),
        (r"Japanese: 日本語", "japanese"),
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

    const ORDINAL_NUMBERS: [(&str, &str); 61] = [
        ("-1", "-1st"),
        ("-2", "-2nd"),
        ("-3", "-3rd"),
        ("-4", "-4th"),
        ("-5", "-5th"),
        ("-6", "-6th"),
        ("-7", "-7th"),
        ("-8", "-8th"),
        ("-9", "-9th"),
        ("-10", "-10th"),
        ("-11", "-11th"),
        ("-12", "-12th"),
        ("-13", "-13th"),
        ("-14", "-14th"),
        ("-20", "-20th"),
        ("-21", "-21st"),
        ("-22", "-22nd"),
        ("-23", "-23rd"),
        ("-24", "-24th"),
        ("-100", "-100th"),
        ("-101", "-101st"),
        ("-102", "-102nd"),
        ("-103", "-103rd"),
        ("-104", "-104th"),
        ("-110", "-110th"),
        ("-111", "-111th"),
        ("-112", "-112th"),
        ("-113", "-113th"),
        ("-1000", "-1000th"),
        ("-1001", "-1001st"),
        ("0", "0th"),
        ("1", "1st"),
        ("2", "2nd"),
        ("3", "3rd"),
        ("4", "4th"),
        ("5", "5th"),
        ("6", "6th"),
        ("7", "7th"),
        ("8", "8th"),
        ("9", "9th"),
        ("10", "10th"),
        ("11", "11th"),
        ("12", "12th"),
        ("13", "13th"),
        ("14", "14th"),
        ("20", "20th"),
        ("21", "21st"),
        ("22", "22nd"),
        ("23", "23rd"),
        ("24", "24th"),
        ("100", "100th"),
        ("101", "101st"),
        ("102", "102nd"),
        ("103", "103rd"),
        ("104", "104th"),
        ("110", "110th"),
        ("111", "111th"),
        ("112", "112th"),
        ("113", "113th"),
        ("1000", "1000th"),
        ("1001", "1001st"),
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
    fn substring_macro() {
        assert_eq!(substr!("1Hello".to_string(), 1), "Hello");
        assert_eq!(substr!("1Hello", 1), "Hello");
        assert_eq!(substr!("1Help-o", 1, 5), "Help");
        assert_eq!(substr!("", 2, 42), "");
        assert_eq!(substr!("<secret>42</secret>", 8, 10), "42");
        assert_eq!(substr!("<secret>42</secret>", 8, 2, true), "42");
    }

    #[test]
    fn camelize() {
        let mut inflection = Inflection::new();

        assert_eq!(inflection.camelize_upper("Capital", false), "capital");
        assert_eq!(inflection.camelize("Camel_Case"), "CamelCase");
        assert_eq!(inflection.camelize("special_guest"), "SpecialGuest");
    }

    #[test]
    fn dasherize() {
        let mut inflection = Inflection::new();
        assert_eq!(inflection.dasherize("puni_puni"), "puni-puni");
    }

    #[test]
    fn humanize() {
        let mut inflection = Inflection::new();
        assert_eq!(inflection.humanize("employee_salary"), "Employee salary");
        assert_eq!(inflection.humanize("author_id"), "Author");
    }

    #[test]
    fn ordinal_unsigned() {
        let inflection = Inflection::new();
        assert_eq!(inflection.ordinal_unsigned(1), "st");
        assert_eq!(inflection.ordinal_unsigned(2), "nd");
        assert_eq!(inflection.ordinal_unsigned(1002), "nd");
        assert_eq!(inflection.ordinal_unsigned(1003), "rd");
    }

    #[test]
    fn ordinal() {
        let inflection = Inflection::new();
        assert_eq!(inflection.ordinal(1), "st");
        assert_eq!(inflection.ordinal(2), "nd");
        assert_eq!(inflection.ordinal(1002), "nd");
        assert_eq!(inflection.ordinal(1003), "rd");
        assert_eq!(inflection.ordinal(-11), "th");
        assert_eq!(inflection.ordinal(-1021), "st");
    }
    #[test]
    fn ordinalize_unsiged() {
        let inflection = Inflection::new();
        assert_eq!(inflection.ordinalize_unsigned(1), "1st");
        assert_eq!(inflection.ordinalize_unsigned(2), "2nd");
        assert_eq!(inflection.ordinalize_unsigned(1002), "1002nd");
        assert_eq!(inflection.ordinalize_unsigned(1003), "1003rd");
    }
    #[test]
    fn ordinalize() {
        let inflection = Inflection::new();
        assert_eq!(inflection.ordinalize(1), "1st");
        assert_eq!(inflection.ordinalize(2), "2nd");
        assert_eq!(inflection.ordinalize(1002), "1002nd");
        assert_eq!(inflection.ordinalize(1003), "1003rd");
        assert_eq!(inflection.ordinalize(-11), "-11th");
        assert_eq!(inflection.ordinalize(-1021), "-1021st");
    }

    #[test]
    fn parameterize() {
        let mut inflection = Inflection::new();
        assert_eq!(
            inflection.parameterize_with_sep(r"Donald E. Knuth", "+".to_string()),
            "donald+e+knuth"
        );
        assert_eq!(
            inflection.parameterize_with_sep(r"Donald E. Knuth", "~".to_string()),
            "donald~e~knuth"
        );
        assert_eq!(
            inflection.parameterize_with_sep(r"Donald E. Knuth", "-".to_string()),
            "donald-e-knuth"
        );
        assert_eq!(
            inflection.parameterize(r"Donald E. Knuth"),
            "donald-e-knuth"
        );
    }

    #[test]
    fn underscore() {
        let mut inflection = Inflection::new();
        assert_eq!(inflection.underscore("DeviceType"), "device_type");

        let rev = inflection.underscore("IOError");
        assert_eq!(inflection.camelize(rev), "IoError");
    }

    #[test]
    fn pluralize() {
        let mut inflection = Inflection::new();
        assert_eq!(inflection.pluralize("post"), "posts");
        assert_eq!(inflection.pluralize("posts"), "posts");
        assert_eq!(inflection.pluralize("octopus"), "octopi");
        assert_eq!(inflection.pluralize("sheep"), "sheep");
        assert_eq!(inflection.pluralize("CamelOctopus"), "CamelOctopi");
        assert_eq!(inflection.pluralize("moves"), "moves");
        assert_eq!(inflection.pluralize("move"), "moves");
        assert_eq!(inflection.pluralize("cow"), "kine");
        assert_eq!(inflection.pluralize("kine"), "kine");
    }

    #[test]
    fn singularize() {
        let mut inflection = Inflection::new();
        assert_eq!(inflection.singularize("post"), "post");
        assert_eq!(inflection.singularize("posts"), "post");
        assert_eq!(inflection.singularize("octopi"), "octopus");
        assert_eq!(inflection.singularize("sheep"), "sheep");
        assert_eq!(inflection.singularize("CamelOctopi"), "CamelOctopus");
        assert_eq!(inflection.singularize("move"), "move");
        assert_eq!(inflection.singularize("moves"), "move");
        assert_eq!(inflection.singularize("cow"), "cow");
        assert_eq!(inflection.singularize("kine"), "cow");
    }

    #[test]
    fn titleize() {
        let mut inflection = Inflection::new();
        assert_eq!(
            inflection.titleize("TheManWithoutAPast"),
            "The Man Without A Past"
        );
        assert_eq!(
            inflection.titleize("x-men: the last stand"),
            "X Men: The Last Stand"
        );
        assert_eq!(
            inflection.titleize("raiders_of_the_lost_ark"),
            "Raiders Of The Lost Ark"
        );
        assert_eq!(
            inflection.titleize("man from the boondocks"),
            "Man From The Boondocks"
        );
    }

    #[test]
    fn camelize_bulk() {
        let mut inflection = Inflection::new();
        for (expected, input) in CAMEL_TO_UNDERSCORE {
            assert_eq!(inflection.camelize(input), expected);
        }
    }

    #[test]
    fn ordinalize_bulk() {
        let inflection = Inflection::new();

        for (input, expected) in ORDINAL_NUMBERS {
            let n: i128 = input.to_string().parse::<i128>().unwrap();
            let expected_u: String = if n < 0 {
                substr!(expected.to_string(), 1).to_string()
            } else {
                expected.to_string()
            };

            assert_eq!(inflection.ordinalize(n), expected);
            assert_eq!(inflection.ordinalize_unsigned(n.abs() as usize), expected_u);
        }
    }

    #[test]
    fn pluralize_bulk() {
        let mut inflection = Inflection::new();
        for (input, expected) in SINGULAR_TO_PLURAL {
            assert_eq!(inflection.pluralize(input), expected);
        }
    }

    #[test]
    fn singularize_bulk() {
        let mut inflection = Inflection::new();
        for (expected, input) in SINGULAR_TO_PLURAL {
            assert_eq!(inflection.singularize(input), expected);
        }
    }

    #[test]
    fn underscore_bulk() {
        let mut inflection = Inflection::new();
        for (expected, input) in UNDERSCORES_TO_DASHES {
            assert_eq!(inflection.underscore(input), expected);
        }
    }

    #[test]
    fn dasherize_bulk() {
        let mut inflection = Inflection::new();
        for (input, expected) in UNDERSCORES_TO_DASHES {
            assert_eq!(inflection.dasherize(input), expected);
        }
    }

    #[test]
    fn tableize_bulk() {
        let mut inflection = Inflection::new();
        for (input, expected) in STRING_TO_TABLEIZE {
            assert_eq!(inflection.tableize(input), expected);
        }

        // for (input, expected) in MIXTURE_TO_TITLEIZED {
        //     assert_eq!(inflection.tableize(input), expected);
        // }
    }

    #[test]
    fn humanize_bulk() {
        let mut inflection = Inflection::new();
        for (input, expected) in UNDERSCORE_TO_HUMAN {
            assert_eq!(inflection.humanize(input), expected);
        }
    }

    // #[test]
    // fn parameterize_bulk() {

    //     let mut inflection = Inflection::new();
    //     for (input, expected) in STRING_TO_PARAMETERIZED_AND_NORMALIZED {
    //         assert_eq!(inflection.parameterize(input), expected);
    //     }
    // }
}
