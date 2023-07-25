use super::*;
use crate::tag::{TagFilter, TagPredicate};

#[test]
fn test_migrate() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
}

#[test]
fn can_manage_files() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    let hash = Hash::new(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]);
    database.hash_add(&hash).unwrap();
    database.hash_remove(&hash).unwrap();
}

#[test]
fn tags_initially_empty() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();

    // no tags initially
    let list = database.tag_list(None, None).unwrap();
    assert_eq!(list, [].into());
}

#[test]
fn can_tags_list_all() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();

    // add tags
    database.tag_add("test", "label").unwrap();
    database.tag_add("test", "other").unwrap();
    database.tag_add("height", "zero").unwrap();
    database.tag_add("height", "other").unwrap();

    // get all tags
    let list = database.tag_list(None, None).unwrap();
    assert_eq!(
        list,
        [
            Tag::new("test".into(), "label".into()),
            Tag::new("test".into(), "other".into()),
            Tag::new("height".into(), "zero".into()),
            Tag::new("height".into(), "other".into())
        ]
        .into()
    );
}

#[test]
fn can_tags_list_by_name() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();

    // add tags
    database.tag_add("test", "label").unwrap();
    database.tag_add("test", "other").unwrap();
    database.tag_add("height", "zero").unwrap();
    database.tag_add("height", "other").unwrap();

    // list by name
    let list = database.tag_list(Some("test"), None).unwrap();
    assert_eq!(
        list,
        [
            Tag::new("test".into(), "label".into()),
            Tag::new("test".into(), "other".into())
        ]
        .into()
    );
}

#[test]
fn can_tags_list_by_value() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();

    // add tags
    database.tag_add("test", "label").unwrap();
    database.tag_add("test", "other").unwrap();
    database.tag_add("height", "zero").unwrap();
    database.tag_add("height", "other").unwrap();

    // list by value
    let list = database.tag_list(None, Some("other")).unwrap();
    assert_eq!(
        list,
        [
            Tag::new("test".into(), "other".into()),
            Tag::new("height".into(), "other".into())
        ]
        .into()
    );
}

#[test]
fn can_tags_rename_by_name() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();

    // add tags
    database.tag_add("test", "label").unwrap();
    database.tag_add("test", "other").unwrap();
    database.tag_add("height", "zero").unwrap();
    database.tag_add("height", "other").unwrap();

    // rename test to foorbar
    database.tag_name_rename("test", "foobar").unwrap();

    // get all tags
    let list = database.tag_list(None, None).unwrap();
    assert_eq!(
        list,
        [
            Tag::new("foobar".into(), "label".into()),
            Tag::new("foobar".into(), "other".into()),
            Tag::new("height".into(), "zero".into()),
            Tag::new("height".into(), "other".into())
        ]
        .into()
    );
}

#[test]
fn can_tags_rename_by_value() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();

    // add tags
    database.tag_add("test", "label").unwrap();
    database.tag_add("test", "other").unwrap();
    database.tag_add("height", "zero").unwrap();
    database.tag_add("height", "other").unwrap();

    // rename test to foorbar
    database.tag_value_rename("height", "other", "new").unwrap();

    // get all tags
    let list = database.tag_list(None, None).unwrap();
    assert_eq!(
        list,
        [
            Tag::new("test".into(), "label".into()),
            Tag::new("test".into(), "other".into()),
            Tag::new("height".into(), "zero".into()),
            Tag::new("height".into(), "new".into())
        ]
        .into()
    );
}

#[test]
fn can_tags_delete_all() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();

    // add tags
    database.tag_add("test", "label").unwrap();
    database.tag_add("test", "other").unwrap();
    database.tag_add("height", "zero").unwrap();
    database.tag_add("height", "other").unwrap();

    database.tag_delete(None, None).unwrap();

    // get all tags
    let list = database.tag_list(None, None).unwrap();
    assert_eq!(list, [].into());
}

#[test]
fn can_tags_delete_by_name() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();

    // add tags
    database.tag_add("test", "label").unwrap();
    database.tag_add("test", "other").unwrap();
    database.tag_add("height", "zero").unwrap();
    database.tag_add("height", "other").unwrap();

    database.tag_delete(Some("test"), None).unwrap();

    // get all tags
    let list = database.tag_list(None, None).unwrap();
    assert_eq!(
        list,
        [
            Tag::new("height".into(), "zero".into()),
            Tag::new("height".into(), "other".into())
        ]
        .into()
    );
}

#[test]
fn can_tags_delete_by_value() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();

    // add tags
    database.tag_add("test", "label").unwrap();
    database.tag_add("test", "other").unwrap();
    database.tag_add("height", "zero").unwrap();
    database.tag_add("height", "other").unwrap();

    // delete tags by value
    database.tag_delete(None, Some("other")).unwrap();

    // get all tags
    let list = database.tag_list(None, None).unwrap();
    assert_eq!(
        list,
        [
            Tag::new("height".into(), "zero".into()),
            Tag::new("test".into(), "label".into())
        ]
        .into()
    );
}

#[test]
fn can_manage_file_tags() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    let hash1 = Hash::new(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]);
    let hash2 = Hash::new(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0x01]);
    database.hash_add(&hash1).unwrap();
    database.hash_add(&hash2).unwrap();
    database.tag_add("name", "value").unwrap();
    database.tag_add("name", "other").unwrap();
    database.hash_tag_add(&hash1, "name", "value").unwrap();
    let tags = database.hash_tags(&hash1, None, None).unwrap();
    assert_eq!(tags, [Tag::new("name".into(), "value".into()),].into());
    assert_eq!(database.hash_tags(&hash2, None, None).unwrap(), [].into());
}

#[test]
fn can_delete_file_tags_all() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    let hash1 = Hash::new(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]);
    database.hash_add(&hash1).unwrap();
    database.tag_add("name", "value").unwrap();
    database.tag_add("name", "other").unwrap();
    database.tag_add("other", "value").unwrap();
    database.tag_add("other", "that").unwrap();
    database.hash_tag_add(&hash1, "name", "value").unwrap();
    database.hash_tag_add(&hash1, "other", "value").unwrap();
    database.hash_tag_add(&hash1, "other", "that").unwrap();
    database.hash_tag_remove(&hash1, None, None).unwrap();
    let tags = database.hash_tags(&hash1, None, None).unwrap();
    assert_eq!(tags, [].into());
}

#[test]
fn can_delete_file_tags_name() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    let hash1 = Hash::new(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]);
    database.hash_add(&hash1).unwrap();
    database.tag_add("name", "value").unwrap();
    database.tag_add("name", "other").unwrap();
    database.tag_add("other", "value").unwrap();
    database.tag_add("other", "that").unwrap();
    database.hash_tag_add(&hash1, "name", "value").unwrap();
    database.hash_tag_add(&hash1, "other", "value").unwrap();
    database.hash_tag_add(&hash1, "other", "that").unwrap();
    database
        .hash_tag_remove(&hash1, Some("name"), None)
        .unwrap();
    assert_eq!(
        database.hash_tags(&hash1, None, None).unwrap(),
        [
            Tag::new("other".into(), "value".into()),
            Tag::new("other".into(), "that".into()),
        ]
        .into()
    );
    database
        .hash_tag_remove(&hash1, Some("other"), None)
        .unwrap();
    assert_eq!(database.hash_tags(&hash1, None, None).unwrap(), [].into());
}

#[test]
fn can_delete_file_tags_individual() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    let hash1 = Hash::new(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]);
    database.hash_add(&hash1).unwrap();
    database.tag_add("name", "value").unwrap();
    database.tag_add("name", "other").unwrap();
    database.tag_add("other", "value").unwrap();
    database.tag_add("other", "that").unwrap();
    database.hash_tag_add(&hash1, "name", "value").unwrap();
    database.hash_tag_add(&hash1, "other", "value").unwrap();
    database.hash_tag_add(&hash1, "other", "that").unwrap();
    database
        .hash_tag_remove(&hash1, Some("other"), Some("value"))
        .unwrap();
    assert_eq!(
        database.hash_tags(&hash1, None, None).unwrap(),
        [
            Tag::new("name".into(), "value".into()),
            Tag::new("other".into(), "that".into()),
        ]
        .into()
    );
    database
        .hash_tag_remove(&hash1, Some("name"), Some("value"))
        .unwrap();
    assert_eq!(
        database.hash_tags(&hash1, None, None).unwrap(),
        [Tag::new("other".into(), "that".into()),].into()
    );
    database
        .hash_tag_remove(&hash1, Some("other"), Some("that"))
        .unwrap();
    assert_eq!(database.hash_tags(&hash1, None, None).unwrap(), [].into());
}

#[test]
fn empty_query_list_returns_all() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    let hash1 = Hash::new(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]);
    let hash2 = Hash::new(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0x04]);
    database.hash_add(&hash1).unwrap();
    database.hash_add(&hash2).unwrap();

    let hashes = database.hash_query(&mut [].iter()).unwrap();
    assert_eq!(hashes, [hash1.into(), hash2.into()].into());
}

#[test]
fn can_query_files_by_tag_name() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    let hash1 = Hash::new(&[0x01]);
    let hash2 = Hash::new(&[0x02]);
    let hash3 = Hash::new(&[0x03]);
    database.hash_add(&hash1).unwrap();
    database.hash_add(&hash2).unwrap();
    database.hash_add(&hash3).unwrap();
    database.tag_add("a", "value").unwrap();
    database.tag_add("b", "value").unwrap();
    database.hash_tag_add(&hash1, "a", "value").unwrap();
    database.hash_tag_add(&hash2, "b", "value").unwrap();
    database.hash_tag_add(&hash3, "a", "value").unwrap();
    database.hash_tag_add(&hash3, "b", "value").unwrap();

    let hashes = database
        .hash_query(&mut [TagPredicate::Exists(TagFilter::new(Some("a"), None))].iter())
        .unwrap();
    assert_eq!(hashes, [hash1.into(), hash3.into()].into());

    let hashes = database
        .hash_query(&mut [TagPredicate::Exists(TagFilter::new(Some("b"), None))].iter())
        .unwrap();
    assert_eq!(hashes, [hash2.into(), hash3.into()].into());

    let hashes = database
        .hash_query(
            &mut [
                TagPredicate::Exists(TagFilter::new(Some("a"), None)),
                TagPredicate::Exists(TagFilter::new(Some("b"), None)),
            ]
            .iter(),
        )
        .unwrap();
    assert_eq!(hashes, [hash3.into()].into());

    let hashes = database
        .hash_query(
            &mut [
                TagPredicate::Exists(TagFilter::new(Some("a"), None)),
                TagPredicate::Missing(TagFilter::new(Some("b"), None)),
            ]
            .iter(),
        )
        .unwrap();
    assert_eq!(hashes, [hash1.into()].into());

    let hashes = database
        .hash_query(
            &mut [
                TagPredicate::Missing(TagFilter::new(Some("a"), None)),
                TagPredicate::Exists(TagFilter::new(Some("b"), None)),
            ]
            .iter(),
        )
        .unwrap();
    assert_eq!(hashes, [hash2.into()].into());
}

#[test]
fn can_query_files_by_tag_name_value() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    let hash1 = Hash::new(&[0x01]);
    let hash2 = Hash::new(&[0x02]);
    database.hash_add(&hash1).unwrap();
    database.hash_add(&hash2).unwrap();
    database.tag_add("name", "a").unwrap();
    database.tag_add("name", "b").unwrap();
    database.tag_add("name", "c").unwrap();
    database.hash_tag_add(&hash1, "name", "a").unwrap();
    database.hash_tag_add(&hash1, "name", "b").unwrap();
    database.hash_tag_add(&hash2, "name", "b").unwrap();
    database.hash_tag_add(&hash2, "name", "c").unwrap();

    let hashes = database
        .hash_query(
            &mut [TagPredicate::Exists(TagFilter::new(
                Some("name"),
                Some("a"),
            ))]
            .iter(),
        )
        .unwrap();
    assert_eq!(hashes, [hash1.into()].into());

    let hashes = database
        .hash_query(
            &mut [TagPredicate::Exists(TagFilter::new(
                Some("name"),
                Some("b"),
            ))]
            .iter(),
        )
        .unwrap();
    assert_eq!(hashes, [hash1.into(), hash2.into()].into());

    let hashes = database
        .hash_query(
            &mut [TagPredicate::Exists(TagFilter::new(
                Some("name"),
                Some("c"),
            ))]
            .iter(),
        )
        .unwrap();
    assert_eq!(hashes, [hash2.into()].into());

    let hashes = database
        .hash_query(
            &mut [
                TagPredicate::Exists(TagFilter::new(Some("name"), Some("a"))),
                TagPredicate::Exists(TagFilter::new(Some("name"), Some("b"))),
            ]
            .iter(),
        )
        .unwrap();
    assert_eq!(hashes, [hash1.into()].into());

    let hashes = database
        .hash_query(
            &mut [
                TagPredicate::Exists(TagFilter::new(Some("name"), Some("b"))),
                TagPredicate::Exists(TagFilter::new(Some("name"), Some("c"))),
            ]
            .iter(),
        )
        .unwrap();
    assert_eq!(hashes, [hash2.into()].into());

    let hashes = database
        .hash_query(
            &mut [
                TagPredicate::Exists(TagFilter::new(Some("name"), Some("a"))),
                TagPredicate::Exists(TagFilter::new(Some("name"), Some("b"))),
                TagPredicate::Exists(TagFilter::new(Some("name"), Some("c"))),
            ]
            .iter(),
        )
        .unwrap();
    assert_eq!(hashes, [].into());

    let hashes = database
        .hash_query(
            &mut [TagPredicate::Missing(TagFilter::new(
                Some("name"),
                Some("b"),
            ))]
            .iter(),
        )
        .unwrap();
    assert_eq!(hashes, [].into());

    let hashes = database
        .hash_query(
            &mut [TagPredicate::Missing(TagFilter::new(
                Some("name"),
                Some("a"),
            ))]
            .iter(),
        )
        .unwrap();
    assert_eq!(hashes, [hash2.into()].into());
}

#[test]
fn stress_test() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    //let transaction = database.transaction().unwrap();
    // create hashes
    let hash = (0..10000)
        .map(|i| format!("{i}").into_bytes())
        .collect::<Vec<_>>();
    for hash in hash.iter() {
        database.hash_add(Hash::new(&hash)).unwrap();
    }
    let tags = [("a", 5), ("b", 7), ("c", 13)]
        .into_iter()
        .map(|(name, values)| {
            (
                name,
                (0..values)
                    .map(|v| format!("value-{v}"))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    for (name, values) in tags.iter() {
        for value in values.iter() {
            database.tag_add(name, &value).unwrap();
        }
        for (hash, value) in hash.iter().zip(values.iter().cycle()) {
            database
                .hash_tag_add(Hash::new(&hash), &name, &value)
                .unwrap();
        }
    }
    let hashes = database.hash_query(&mut [].iter()).unwrap();
    assert_eq!(hashes.len(), hash.len());

    for (name, values) in tags.iter() {
        let hashes = database
            .hash_query(
                &mut [TagPredicate::Exists(TagFilter::new(
                    Some(*name),
                    Some(&values[0]),
                ))]
                .iter(),
            )
            .unwrap();
        assert_eq!(
            hashes,
            hash.iter()
                .step_by(values.len())
                .map(|v| Box::<[u8]>::from(v[..].to_vec()))
                .map(BoxHash::from)
                .collect()
        );
    }

    let _hashes = database
        .hash_query(&mut [TagPredicate::Exists(TagFilter::new(None, Some("value-3")))].iter())
        .unwrap();
}
