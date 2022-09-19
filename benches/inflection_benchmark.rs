use inflection_rs::inflection::Inflection;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_press(c: &mut Criterion) {
    c.bench_function("camelize", |b| {
        b.iter(|| {
            let mut inflection = black_box(Inflection::new());
            assert_eq!(inflection.camelize_upper("Capital", false), "capital");
            assert_eq!(inflection.camelize("Camel_Case"), "CamelCase");
            assert_eq!(inflection.camelize("special_guest"), "SpecialGuest");
        })
    });

    c.bench_function("dasherize", |b| {
        b.iter(|| {
            let mut inflection = black_box(Inflection::new());
            assert_eq!(inflection.dasherize("puni_puni"), "puni-puni");
        })
    });

    c.bench_function("parameterize", |b| {
        b.iter(|| {
            let mut inflection = black_box(Inflection::new());
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
        })
    });

    c.bench_function("underscore", |b| {
        b.iter(|| {
            let mut inflection = black_box(Inflection::new());

            assert_eq!(inflection.underscore("DeviceType"), "device_type");
            // Note: not always reversible
            let rev = inflection.underscore("IOError");
            assert_eq!(inflection.camelize(rev), "IoError");
        })
    });

    c.bench_function("pluralize", |b| {
        b.iter(|| {
            let mut inflection = black_box(Inflection::new());

            assert_eq!(inflection.pluralize("post"), "posts");
            assert_eq!(inflection.pluralize("posts"), "posts");
            assert_eq!(inflection.pluralize("octopus"), "octopi");
            assert_eq!(inflection.pluralize("sheep"), "sheep");
            assert_eq!(inflection.pluralize("CamelOctopus"), "CamelOctopi");
            assert_eq!(inflection.pluralize("moves"), "moves");
            assert_eq!(inflection.pluralize("move"), "moves");
            assert_eq!(inflection.pluralize("cow"), "kine");
            assert_eq!(inflection.pluralize("kine"), "kine");
            assert_eq!(inflection.pluralize("slave"), "slaves");
            assert_eq!(inflection.pluralize("slaves"), "slaves");
        })
    });

    c.bench_function("singularize", |b| {
        b.iter(|| {
            let mut inflection = black_box(Inflection::new());

            assert_eq!(inflection.singularize("post"), "post");
            assert_eq!(inflection.singularize("posts"), "post");
            assert_eq!(inflection.singularize("octopi"), "octopus");
            assert_eq!(inflection.singularize("sheep"), "sheep");
            assert_eq!(inflection.singularize("CamelOctopi"), "CamelOctopus");
            assert_eq!(inflection.singularize("move"), "move");
            assert_eq!(inflection.singularize("moves"), "move");
            assert_eq!(inflection.singularize("cow"), "cow");
            assert_eq!(inflection.singularize("kine"), "cow");
            assert_eq!(inflection.singularize("slaves"), "slave");
            assert_eq!(inflection.singularize("slave"), "slave");
        })
    });

    c.bench_function("titleize", |b| {
        b.iter(|| {
            let mut inflection = black_box(Inflection::new());

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
        })
    });

    c.bench_function("ordinal_integer", |b| {
        b.iter(|| {
            let inflection = black_box(Inflection::new());

            assert_eq!(inflection.ordinal_u8(1), "st");
            assert_eq!(inflection.ordinal_u16(2), "nd");
            assert_eq!(inflection.ordinal_u32(3), "rd");
            assert_eq!(inflection.ordinal_u64(4), "th");
            assert_eq!(inflection.ordinal_u128(10), "th");
            assert_eq!(inflection.ordinal_usize(10), "th");

            assert_eq!(inflection.ordinal_i8(1), "st");
            assert_eq!(inflection.ordinal_i16(2), "nd");
            assert_eq!(inflection.ordinal_i32(3), "rd");
            assert_eq!(inflection.ordinal_i64(4), "th");
            assert_eq!(inflection.ordinal_i128(10), "th");

            assert_eq!(inflection.ordinal_i8(-1), "st");
            assert_eq!(inflection.ordinal_i16(-2), "nd");
            assert_eq!(inflection.ordinal_i32(-3), "rd");
            assert_eq!(inflection.ordinal_i64(-4), "th");
            assert_eq!(inflection.ordinal_i128(-10), "th");
        })
    });

    c.bench_function("ordinalize_integer", |b| {
        b.iter(|| {
            let inflection = black_box(Inflection::new());

            assert_eq!(inflection.ordinalize_u8(1), "1st");
            assert_eq!(inflection.ordinalize_u16(2), "2nd");
            assert_eq!(inflection.ordinalize_u32(3), "3rd");
            assert_eq!(inflection.ordinalize_u64(4), "4th");
            assert_eq!(inflection.ordinalize_u128(10), "10th");
            assert_eq!(inflection.ordinalize_usize(100), "100th");
            assert_eq!(inflection.ordinalize_usize(10000000), "10000000th");

            assert_eq!(inflection.ordinalize_i8(1), "1st");
            assert_eq!(inflection.ordinalize_i16(2), "2nd");
            assert_eq!(inflection.ordinalize_i32(3), "3rd");
            assert_eq!(inflection.ordinalize_i64(4), "4th");
            assert_eq!(inflection.ordinalize_i128(10), "10th");
            assert_eq!(inflection.ordinalize_i128(10000000), "10000000th");

            assert_eq!(inflection.ordinalize_i8(-1), "-1st");
            assert_eq!(inflection.ordinalize_i16(-2), "-2nd");
            assert_eq!(inflection.ordinalize_i32(-3), "-3rd");
            assert_eq!(inflection.ordinalize_i64(-4), "-4th");
            assert_eq!(inflection.ordinalize_i128(-10000), "-10000th");
            assert_eq!(inflection.ordinalize_i128(-10000000), "-10000000th");
        })
    });

    c.bench_function("normalize_spaces", |b| {
        b.iter(|| {
            let mut inflection = black_box(Inflection::new());

            assert_eq!(
                inflection.normalize_spaces("   hello     there    "),
                "hello there"
            );
            assert_eq!(
                inflection.normalize_spaces("   hell      o     there    "),
                "hell o there"
            );
            assert_eq!(inflection.normalize_spaces(""), "");
            assert_eq!(inflection.normalize_spaces("   "), "");
        })
    });
}

criterion_group!(benches, bench_press);
criterion_main!(benches);
