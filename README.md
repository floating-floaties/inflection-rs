# Inflection Rust

This crate is a port of a port:

> Inflection is a string transformation library. It singularizes and pluralizes English words, and transforms strings from CamelCase to underscored string. Inflection is a port of Ruby on Rails’ inflector to Python.
> 
> Source: https://github.com/jpvanhal/inflection

## Documentation

### Singularize
```rust
use inflection_rs::inflection;

fn main() {
    assert_eq!(inflection::singularize("post"), "post");
    assert_eq!(inflection::singularize("posts"), "post");
    assert_eq!(inflection::singularize("octopi"), "octopus");
    assert_eq!(inflection::singularize("sheep"), "sheep");
    assert_eq!(inflection::singularize("CamelOctopi"), "CamelOctopus");
    assert_eq!(inflection::singularize("move"), "move");
    assert_eq!(inflection::singularize("moves"), "move");
    assert_eq!(inflection::singularize("cow"), "cow");
    assert_eq!(inflection::singularize("kine"), "cow");
    assert_eq!(inflection::singularize("slaves"), "slave");
    assert_eq!(inflection::singularize("slave"), "slave");
    assert_eq!(inflection::singularize("books"), "book");
    assert_eq!(inflection::singularize("cities"), "city");
    assert_eq!(inflection::singularize("buzzes"), "buzz");
    assert_eq!(inflection::singularize("children"), "child");
    assert_eq!(inflection::singularize("people"), "person");
    assert_eq!(inflection::singularize("mice"), "mouse");
    assert_eq!(inflection::singularize("analyses"), "analysis");
    assert_eq!(inflection::singularize("theses"), "thesis");
    assert_eq!(inflection::singularize("crises"), "crisis");
    assert_eq!(inflection::singularize("radii"), "radius");
    assert_eq!(inflection::singularize("data"), "datum");
    assert_eq!(inflection::singularize("mothers-in-law"), "mother-in-law");
    assert_eq!(inflection::singularize("passersby"), "passerby");
    assert_eq!(inflection::singularize("potatoes"), "potato");
    assert_eq!(inflection::singularize("heroes"), "hero");
    assert_eq!(inflection::singularize("pianos"), "piano");
    assert_eq!(inflection::singularize("piano"), "piano");
    assert_eq!(inflection::singularize("datum"), "datum");
    assert_eq!(inflection::singularize("bookshelves"), "bookshelf");
    assert_eq!(inflection::singularize("merry-go-rounds"), "merry-go-round");
    assert_eq!(inflection::singularize("post offices"), "post office");
}
```

### Pluralize
```rust
use inflection_rs::inflection;

fn main() {
    assert_eq!(inflection::pluralize("post"), "posts");
    assert_eq!(inflection::pluralize("posts"), "posts");
    assert_eq!(inflection::pluralize("octopus"), "octopi");
    assert_eq!(inflection::pluralize("sheep"), "sheep");
    assert_eq!(inflection::pluralize("CamelOctopus"), "CamelOctopi");
    assert_eq!(inflection::pluralize("moves"), "moves");
    assert_eq!(inflection::pluralize("move"), "moves");
    assert_eq!(inflection::pluralize("cow"), "kine");
    assert_eq!(inflection::pluralize("kine"), "kine");
    assert_eq!(inflection::pluralize("slave"), "slaves");
    assert_eq!(inflection::pluralize("slaves"), "slaves");
    assert_eq!(inflection::pluralize("book"), "books");
    assert_eq!(inflection::pluralize("city"), "cities");
    assert_eq!(inflection::pluralize("buzz"), "buzzes");
    assert_eq!(inflection::pluralize("child"), "children");
    assert_eq!(inflection::pluralize("person"), "people");
    assert_eq!(inflection::pluralize("mouse"), "mice");
    assert_eq!(inflection::pluralize("analysis"), "analyses");
    assert_eq!(inflection::pluralize("thesis"), "theses");
    assert_eq!(inflection::pluralize("crisis"), "crises");
    assert_eq!(inflection::pluralize("radius"), "radii");
    assert_eq!(inflection::pluralize("datum"), "data");
    assert_eq!(inflection::pluralize("mother-in-law"), "mothers-in-law");
    assert_eq!(inflection::pluralize("father-in-law"), "fathers-in-law");
    assert_eq!(inflection::pluralize("brother-in-law"), "brothers-in-law");
    assert_eq!(inflection::pluralize("sister-in-law"), "sisters-in-law");
    assert_eq!(inflection::pluralize("passerby"), "passersby");
    assert_eq!(inflection::pluralize("potato"), "potatoes");
    assert_eq!(inflection::pluralize("hero"), "heroes");
    assert_eq!(inflection::pluralize("piano"), "pianos");
    assert_eq!(inflection::pluralize("pianos"), "pianos");
    assert_eq!(inflection::pluralize("data"), "data");
    assert_eq!(inflection::pluralize("bookshelf"), "bookshelves");
    assert_eq!(inflection::pluralize("merry-go-round"), "merry-go-rounds");
    assert_eq!(inflection::pluralize("post office"), "post offices");
}
```

### Keyify

```rust
use inflection_rs::inflection;

fn main() {
    assert_eq!(inflection::keyify(r"!@#Leading bad characters"), "leading_bad_characters");
}

```

### Titleize
```rust
use inflection_rs::inflection;   

fn main() {
    assert_eq!(
        inflection::titleize("TheManWithoutAPast"),
        "The Man Without A Past"
    );
    assert_eq!(
        inflection::titleize("x-men: the last stand"),
        "X Men: The Last Stand"
    );
    assert_eq!(
        inflection::titleize("raiders_of_the_lost_ark"),
        "Raiders Of The Lost Ark"
    );
    assert_eq!(
        inflection::titleize("man from the boondocks"),
        "Man From The Boondocks"
    );
}
```

### Camelize
```rust
use inflection_rs::inflection;

fn main() {
    assert_eq!(inflection::camelize_upper("Capital", false), "capital");
    assert_eq!(inflection::camelize("Camel_Case"), "CamelCase");
    assert_eq!(inflection::camelize("special_guest"), "SpecialGuest");
}
```

### Dasherize
```rust
use inflection_rs::inflection;

fn main() {
    assert_eq!(inflection::dasherize("puni_puni"), "puni-puni");
}
```

### Humanize
```rust
use inflection_rs::inflection;

fn main() {
    assert_eq!(inflection::humanize("employee_salary"), "Employee salary");
    assert_eq!(inflection::humanize("author_id"), "Author");
}
```

### Parameterize
```rust
use inflection_rs::inflection;

fn main() {
    assert_eq!(
        inflection::parameterize_with_sep(r"Donald E. Knuth", "+".to_string()),
        "donald+e+knuth"
    );
    assert_eq!(
        inflection::parameterize_with_sep(r"Donald E. Knuth", "~".to_string()),
        "donald~e~knuth"
    );
    assert_eq!(
        inflection::parameterize_with_sep(r"Donald E. Knuth", "-".to_string()),
        "donald-e-knuth"
    );
    assert_eq!(
        inflection::parameterize(r"Donald E. Knuth"),
        "donald-e-knuth"
    );
}
```

### Underscore
```rust
use inflection_rs::inflection;

fn main() {
    assert_eq!(inflection::underscore("DeviceType"), "device_type");
    // Note: not always reversible
    let rev = inflection::underscore("IOError");
    assert_eq!(inflection::camelize(rev), "IoError");
}
```

### Ordinal

```rust
use inflection_rs::inflection;   

fn main() {
    assert_eq!(inflection::ordinal_u8(1), "st");
    assert_eq!(inflection::ordinal_u16(2), "nd");
    assert_eq!(inflection::ordinal_u32(3), "rd");
    assert_eq!(inflection::ordinal_u64(4), "th");
    assert_eq!(inflection::ordinal_u128(10), "th");
    assert_eq!(inflection::ordinal_usize(10), "th");

    assert_eq!(inflection::ordinal_i8(1), "st");
    assert_eq!(inflection::ordinal_i16(2), "nd");
    assert_eq!(inflection::ordinal_i32(3), "rd");
    assert_eq!(inflection::ordinal_i64(4), "th");
    assert_eq!(inflection::ordinal_i128(10), "th");

    assert_eq!(inflection::ordinal_i8(-1), "st");
    assert_eq!(inflection::ordinal_i16(-2), "nd");
    assert_eq!(inflection::ordinal_i32(-3), "rd");
    assert_eq!(inflection::ordinal_i64(-4), "th");
    assert_eq!(inflection::ordinal_i128(-10), "th");
}
```


### Ordinalize

```rust
use inflection_rs::inflection;   

fn main() {
    assert_eq!(inflection::ordinalize_u8(1), "1st");
    assert_eq!(inflection::ordinalize_u16(2), "2nd");
    assert_eq!(inflection::ordinalize_u32(3), "3rd");
    assert_eq!(inflection::ordinalize_u64(4), "4th");
    assert_eq!(inflection::ordinalize_u128(10), "10th");
    assert_eq!(inflection::ordinalize_usize(100), "100th");
    assert_eq!(inflection::ordinalize_usize(10000000), "10000000th");

    assert_eq!(inflection::ordinalize_i8(1), "1st");
    assert_eq!(inflection::ordinalize_i16(2), "2nd");
    assert_eq!(inflection::ordinalize_i32(3), "3rd");
    assert_eq!(inflection::ordinalize_i64(4), "4th");
    assert_eq!(inflection::ordinalize_i128(10), "10th");
    assert_eq!(inflection::ordinalize_i128(10000000), "10000000th");

    assert_eq!(inflection::ordinalize_i8(-1), "-1st");
    assert_eq!(inflection::ordinalize_i16(-2), "-2nd");
    assert_eq!(inflection::ordinalize_i32(-3), "-3rd");
    assert_eq!(inflection::ordinalize_i64(-4), "-4th");
    assert_eq!(inflection::ordinalize_i128(-10000), "-10000th");
    assert_eq!(inflection::ordinalize_i128(-10000000), "-10000000th");
}
```

### Normalize Spaces
```rust
use inflection_rs::inflection;

fn main() {
    assert_eq!(inflection::normalize_spaces("   hello     there    "), "hello there");
    assert_eq!(inflection::normalize_spaces("   hell      o     there    "), "hell o there");
    assert_eq!(inflection::normalize_spaces(""), "");
    assert_eq!(inflection::normalize_spaces("   "), "");
}
```
