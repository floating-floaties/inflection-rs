# Inflection Rust

This crate is a port of a port:

> Inflection is a string transformation library. It singularizes and pluralizes English words, and transforms strings from CamelCase to underscored string. Inflection is a port of Ruby on Rails’ inflector to Python.
> 
> Source: https://github.com/jpvanhal/inflection

## Crate

```toml
inflection-rs = "^0.1"
```

## Documentation

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
}
```

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

## Benchmarks
<pre><font color="#4E9A06">Benchmarking camelize: Collecting 100 samples in estimated 5.0131 s (1.3M iteratcamelize</font>                
time:   [3.8203 µs <b>3.8822 µs</b> 3.9575 µs]
                        change: [-3.3200% -2.0717% -0.7912%] (p = 0.00 &lt; 0.05)
                        Change within noise threshold.
<font color="#C4A000">Found 22 outliers among 100 measurements (22.00%)</font>
  4 (4.00%) high mild
  18 (18.00%) high severe

<font color="#4E9A06">Benchmarking dasherize: Collecting 100 samples in estimated 5.0002 s (93M iteratdasherize</font>               
time:   [48.364 ns <b>48.842 ns</b> 49.432 ns]
                        change: [-0.4351% +0.3857% +1.3238%] (p = 0.36 &gt; 0.05)
                        No change in performance detected.
<font color="#C4A000">Found 12 outliers among 100 measurements (12.00%)</font>
  4 (4.00%) high mild
  8 (8.00%) high severe

<font color="#4E9A06">Benchmarking parameterize: Collecting 100 samples in estimated 5.0084 s (530k itparameterize</font>            
time:   [8.8806 µs <b>8.9130 µs</b> 8.9508 µs]
                        change: [-0.9482% -0.4425% +0.0430%] (p = 0.08 &gt; 0.05)
                        No change in performance detected.
<font color="#C4A000">Found 9 outliers among 100 measurements (9.00%)</font>
  7 (7.00%) high mild
  2 (2.00%) high severe

<font color="#4E9A06">Benchmarking underscore: Collecting 100 samples in estimated 5.0244 s (480k iterunderscore</font>              
time:   [9.9439 µs <b>10.015 µs</b> 10.112 µs]
                        change: [-3.5014% <font color="#4E9A06"><b>-2.7819%</b></font> -2.0878%] (p = 0.00 &lt; 0.05)
                        Performance has <font color="#4E9A06">improved</font>.
<font color="#C4A000">Found 16 outliers among 100 measurements (16.00%)</font>
  4 (4.00%) low mild
  2 (2.00%) high mild
  10 (10.00%) high severe

<font color="#4E9A06">Benchmarking pluralize: Collecting 100 samples in estimated 5.0145 s (1.0M iterapluralize</font>               
time:   [5.0134 µs <b>5.1127 µs</b> 5.2129 µs]
                        change: [+7.7346% <font color="#CC0000"><b>+8.8241%</b></font> +10.069%] (p = 0.00 &lt; 0.05)
                        Performance has <font color="#CC0000">regressed</font>.
<font color="#C4A000">Found 22 outliers among 100 measurements (22.00%)</font>
  22 (22.00%) high severe

<font color="#4E9A06">Benchmarking singularize: Collecting 100 samples in estimated 5.0067 s (1.9M itesingularize</font>             
time:   [2.7191 µs <b>2.7478 µs</b> 2.7811 µs]
                        change: [+1.9819% <font color="#CC0000"><b>+2.8260%</b></font> +3.7477%] (p = 0.00 &lt; 0.05)
                        Performance has <font color="#CC0000">regressed</font>.
<font color="#C4A000">Found 18 outliers among 100 measurements (18.00%)</font>
  2 (2.00%) high mild
  16 (16.00%) high severe

<font color="#4E9A06">Benchmarking titleize: Collecting 100 samples in estimated 5.1413 s (56k iteratititleize</font>                
time:   [91.735 µs <b>91.828 µs</b> 91.925 µs]
                        change: [-0.2438% +0.1241% +0.4457%] (p = 0.49 &gt; 0.05)
                        No change in performance detected.
<font color="#C4A000">Found 5 outliers among 100 measurements (5.00%)</font>
  3 (3.00%) high mild
  2 (2.00%) high severe

Benchmarking ordinal_integer: Collecting 100 samples in estimated 5.0000 s (288M<font color="#4E9A06">ordinal_integer</font>         
time:   [16.952 ns <b>17.205 ns</b> 17.505 ns]
                        change: [+3.8986% <font color="#CC0000"><b>+5.5305%</b></font> +7.1434%] (p = 0.00 &lt; 0.05)
                        Performance has <font color="#CC0000">regressed</font>.
<font color="#C4A000">Found 4 outliers among 100 measurements (4.00%)</font>
  4 (4.00%) high mild

Benchmarking ordinalize_integer: Collecting 100 samples in estimated 5.0000 s (6<font color="#4E9A06">ordinalize_integer</font>      
time:   [74.002 ns <b>74.839 ns</b> 75.814 ns]
                        change: [+3.7319% <font color="#CC0000"><b>+6.5529%</b></font> +9.6891%] (p = 0.00 &lt; 0.05)
                        Performance has <font color="#CC0000">regressed</font>.
<font color="#C4A000">Found 12 outliers among 100 measurements (12.00%)</font>
  11 (11.00%) high mild
  1 (1.00%) high severe

Benchmarking normalize_spaces: Collecting 100 samples in estimated 5.0147 s (828<font color="#4E9A06">normalize_spaces</font>        
time:   [5.8609 µs <b>5.9367 µs</b> 6.0192 µs]
                        change: [-0.0130% +1.0430% +2.2086%] (p = 0.08 &gt; 0.05)
                        No change in performance detected.
<font color="#C4A000">Found 7 outliers among 100 measurements (7.00%)</font>
  2 (2.00%) high mild
  5 (5.00%) high severe

Benchmarking keyify: Collecting 100 samples in estimated 5.6943 s (40k iteration<font color="#4E9A06">keyify</font>                  
time:   [140.24 µs <b>140.30 µs</b> 140.39 µs]
                        change: [-1.8793% <font color="#4E9A06"><b>-1.4618%</b></font> -1.0779%] (p = 0.00 &lt; 0.05)
                        Performance has <font color="#4E9A06">improved</font>.
<font color="#C4A000">Found 12 outliers among 100 measurements (12.00%)</font>
  3 (3.00%) high mild
  9 (9.00%) high severe</pre>