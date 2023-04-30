use inflection_rs::inflection;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_press(c: &mut Criterion) {
    c.bench_function("camelize", |b| {
        b.iter(|| {
            assert_eq!(
                inflection::camelize(black_box("special_guest")),
                "SpecialGuest"
            );
        })
    });

    c.bench_function("dasherize", |b| {
        b.iter(|| {
            assert_eq!(inflection::dasherize(black_box("puni_puni")), "puni-puni");
        })
    });

    c.bench_function("parameterize", |b| {
        b.iter(|| {
            assert_eq!(
                inflection::parameterize(black_box(r"Donald E. Knuth")),
                "donald-e-knuth"
            );
        })
    });

    c.bench_function("underscore", |b| {
        b.iter(|| {
            assert_eq!(
                inflection::underscore(black_box("DeviceType")),
                "device_type"
            );
        })
    });

    c.bench_function("pluralize", |b| {
        b.iter(|| {
            assert_eq!(
                inflection::pluralize(black_box("CamelOctopus")),
                "CamelOctopi"
            );
        })
    });

    c.bench_function("singularize", |b| {
        b.iter(|| {
            assert_eq!(
                inflection::singularize(black_box("CamelOctopi")),
                "CamelOctopus"
            );
        })
    });

    c.bench_function("titleize", |b| {
        b.iter(|| {
            assert_eq!(
                inflection::titleize(black_box("raiders_of_the_lost_ark")),
                "Raiders Of The Lost Ark"
            );
        })
    });

    c.bench_function("ordinal_integer", |b| {
        b.iter(|| {
            assert_eq!(inflection::ordinal_i128(black_box(-10)), "th");
        })
    });

    c.bench_function("ordinalize_integer", |b| {
        b.iter(|| {
            assert_eq!(
                inflection::ordinalize_i128(black_box(-10000000)),
                "-10000000th"
            );
        })
    });

    c.bench_function("normalize_spaces", |b| {
        b.iter(|| {
            assert_eq!(
                inflection::normalize_spaces("   hello     there    "),
                "hello there"
            );
        })
    });

    c.bench_function("keyify", |b| {
        b.iter(|| {
            assert_eq!(
                inflection::keyify(black_box("   hello     there    ")),
                "hello_there"
            );
        })
    });
}

criterion_group!(benches, bench_press);
criterion_main!(benches);
