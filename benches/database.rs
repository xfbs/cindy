use cindy::{
    config::HashAlgorithm,
    hash::{BoxHash, DataHasher, Hash},
    Database, TagFilter,
};
use criterion::*;
use rand::{thread_rng, Rng};
use rusqlite::Connection;
use std::time::Duration;

/// Create empty, migrated database.
fn database() -> Database {
    let mut database: Database = Connection::open_in_memory().unwrap().into();
    database.migrate().unwrap();
    database
}

/// Create database with sample data.
fn database_full(hashes: u64, cyclic: &[(&str, u64)], random: &[(&str, f64)]) -> Database {
    let mut rng = thread_rng();

    // create database and transaction
    let mut database = database();
    let mut transaction = database.transaction().unwrap();

    // create hashes
    let hash = (0..hashes)
        .map(|i| format!("file-{i}").into_bytes())
        .map(|name| HashAlgorithm::Blake2b512.hash_data(&name[..]))
        .collect::<Vec<BoxHash>>();
    for hash in hash.iter() {
        transaction.hash_add(hash).unwrap();
    }

    // create cyclic tags
    let tags = cyclic
        .into_iter()
        .map(|(name, values)| {
            (
                name,
                (0..*values)
                    .map(|v| format!("value{v}"))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    for (name, values) in tags.iter() {
        transaction.tag_name_create(name, None).unwrap();
        for value in values.iter() {
            transaction.tag_value_create(name, &value).unwrap();
        }
        for (hash, value) in hash.iter().zip(values.iter().cycle()) {
            transaction.hash_tag_add(hash, &name, &value).unwrap();
        }
    }

    // create random tags
    transaction.tag_name_create("random", None).unwrap();
    for (name, probability) in random {
        transaction.tag_value_create("random", name).unwrap();
        for hash in hash.iter() {
            if rng.gen_bool(*probability) {
                transaction.hash_tag_add(&hash, "random", name).unwrap();
            }
        }
    }

    transaction.commit().unwrap();
    database
}

/// Benchmark database insertions.
fn insertions(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert");
    group.warm_up_time(Duration::from_secs(1));

    let count = 10000;
    group.throughput(Throughput::Elements(count));

    group.bench_function("file", move |b| {
        let hash_algorithm = HashAlgorithm::Blake2b512;
        let hashes: Vec<BoxHash> = (0..count)
            .map(|num| format!("file-{num}"))
            .map(|num| hash_algorithm.hash_data(&num.as_bytes()))
            .collect();
        b.iter_batched_ref(
            database,
            |database| {
                let transaction = database.transaction().unwrap();
                for hash in &hashes {
                    transaction.hash_add(&hash).unwrap();
                }
                transaction.commit().unwrap();
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("tag_names", move |b| {
        let hash_algorithm = HashAlgorithm::Blake2b512;
        let names: Vec<String> = (0..count).map(|num| format!("file-{num}")).collect();
        b.iter_batched_ref(
            database,
            |database| {
                let transaction = database.transaction().unwrap();
                for name in &names {
                    transaction.tag_name_create(&name, None).unwrap();
                }
                transaction.commit().unwrap();
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("tag_values", move |b| {
        let hash_algorithm = HashAlgorithm::Blake2b512;
        let names: Vec<String> = (0..count).map(|num| format!("file-{num}")).collect();
        b.iter_batched_ref(
            || {
                let mut database = database();
                database.tag_name_create("test", None).unwrap();
                database
            },
            |database| {
                let transaction = database.transaction().unwrap();
                for name in &names {
                    transaction.tag_value_create("test", &name).unwrap();
                }
                transaction.commit().unwrap();
            },
            BatchSize::SmallInput,
        );
    });
}

fn reading(c: &mut Criterion) {}

/// Benchmark queries.
fn querying(c: &mut Criterion) {
    let mut group = c.benchmark_group("query");
    group.warm_up_time(Duration::from_secs(1));
    group.sample_size(10);

    let count = 100000;
    let database = database_full(
        count,
        &[("tag0", 3), ("tag1", 7), ("tag2", 13), ("tag3", 17)],
        &[("half", 0.5), ("rare", 0.01), ("common", 0.9)],
    );
    group.throughput(Throughput::Elements(count));

    group.bench_function("all", |b| {
        b.iter(|| {
            database.query_hashes(&mut [].iter()).unwrap();
        });
    });

    group.bench_function("tag0:*", |b| {
        b.iter(|| {
            let tags = database
                .query_hashes(&mut [TagFilter::new(Some("tag0"), None).exists()].iter())
                .unwrap();
            assert_eq!(tags.len(), count as usize);
        });
    });

    group.bench_function("tag0:value0", |b| {
        b.iter(|| {
            database
                .query_hashes(&mut [TagFilter::new(Some("tag0"), Some("value0")).exists()].iter())
                .unwrap();
        });
    });

    group.bench_function("tag1:value0", |b| {
        b.iter(|| {
            database
                .query_hashes(&mut [TagFilter::new(Some("tag1"), Some("value0")).exists()].iter())
                .unwrap();
        });
    });

    group.bench_function("tag0:value0+tag1:value0", |b| {
        b.iter(|| {
            database
                .query_hashes(
                    &mut [
                        TagFilter::new(Some("tag0"), Some("value0")).exists(),
                        TagFilter::new(Some("tag1"), Some("value0")).exists(),
                    ]
                    .iter(),
                )
                .unwrap();
        });
    });

    group.bench_function("random:half", |b| {
        b.iter(|| {
            database
                .query_hashes(&mut [TagFilter::new(Some("random"), Some("half")).exists()].iter())
                .unwrap();
        });
    });

    group.bench_function("random:rare", |b| {
        b.iter(|| {
            database
                .query_hashes(&mut [TagFilter::new(Some("random"), Some("rare")).exists()].iter())
                .unwrap();
        });
    });

    group.bench_function("random:common", |b| {
        b.iter(|| {
            database
                .query_hashes(&mut [TagFilter::new(Some("random"), Some("common")).exists()].iter())
                .unwrap();
        });
    });
}

criterion_group!(benches, insertions, querying, reading);
criterion_main!(benches);
