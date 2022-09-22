use inflection_rs::inflection::Inflection;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_press(c: &mut Criterion) {
    let mut inflection = black_box(Inflection::default());

    c.bench_function("camelize", |b| {
        b.iter(|| {
            assert_eq!(inflection.camelize("special_guest"), "SpecialGuest");
        })
    });

    c.bench_function("dasherize", |b| {
        b.iter(|| {
            assert_eq!(inflection.dasherize("puni_puni"), "puni-puni");
        })
    });

    c.bench_function("parameterize", |b| {
        b.iter(|| {
            assert_eq!(
                inflection.parameterize(r"Donald E. Knuth"),
                "donald-e-knuth"
            );
        })
    });

    c.bench_function("underscore", |b| {
        b.iter(|| {
            assert_eq!(inflection.underscore("DeviceType"), "device_type");
        })
    });

    c.bench_function("pluralize", |b| {
        b.iter(|| {
            assert_eq!(inflection.pluralize("CamelOctopus"), "CamelOctopi");
        })
    });

    c.bench_function("singularize", |b| {
        b.iter(|| {
            assert_eq!(inflection.singularize("CamelOctopi"), "CamelOctopus");
        })
    });

    c.bench_function("titleize", |b| {
        b.iter(|| {
            assert_eq!(
                inflection.titleize("raiders_of_the_lost_ark"),
                "Raiders Of The Lost Ark"
            );
        })
    });

    c.bench_function("ordinal_integer", |b| {
        b.iter(|| {
            assert_eq!(inflection.ordinal_i128(-10), "th");
        })
    });

    c.bench_function("ordinalize_integer", |b| {
        b.iter(|| {
            assert_eq!(inflection.ordinalize_i128(-10000000), "-10000000th");
        })
    });

    c.bench_function("normalize_spaces", |b| {
        b.iter(|| {
            assert_eq!(
                inflection.normalize_spaces("   hello     there    "),
                "hello there"
            );
        })
    });
}

criterion_group!(benches, bench_press);
criterion_main!(benches);
