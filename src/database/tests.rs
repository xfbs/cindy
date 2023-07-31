use super::*;
use crate::tag::{TagFilter, TagPredicate, TagValueInfo};
use cindy_common::{Point, Rectangle, Sequence};

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
fn can_tags_list_all_one() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();

    // add tags
    database.tag_add("test", "label").unwrap();

    // get all tags
    let list = database.tag_list(None, None).unwrap();
    assert_eq!(
        list[&Tag::new("test".into(), "label".into())],
        TagValueInfo {
            files: 0,
            display: "label".into(),
            system: false,
        }
    );
}

#[test]
fn can_tags_value_set_display() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();

    // add tags
    database.tag_add("test", "label").unwrap();
    database
        .tag_value_display("test", "label", "Label Name")
        .unwrap();

    // get all tags
    let list = database.tag_list(None, None).unwrap();
    assert_eq!(
        list[&Tag::new("test".into(), "label".into())],
        TagValueInfo {
            files: 0,
            display: "Label Name".into(),
            system: false,
        }
    );
}

#[test]
fn can_tags_list_all_multiple() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();

    // add tags
    database.tag_add("test", "label").unwrap();
    database.tag_add("test", "other").unwrap();
    database.tag_add("name", "zero").unwrap();
    database.tag_add("name", "other").unwrap();

    // get all tags
    let list = database.tag_list(None, None).unwrap();
    assert_eq!(list.len(), 4);
    assert_eq!(
        list[&Tag::new("test".into(), "label".into())],
        TagValueInfo {
            files: 0,
            display: "label".into(),
            system: false,
        }
    );
    assert_eq!(
        list[&Tag::new("test".into(), "other".into())],
        TagValueInfo {
            files: 0,
            display: "other".into(),
            system: false,
        }
    );
    assert_eq!(
        list[&Tag::new("name".into(), "zero".into())],
        TagValueInfo {
            files: 0,
            display: "zero".into(),
            system: false,
        }
    );
    assert_eq!(
        list[&Tag::new("name".into(), "other".into())],
        TagValueInfo {
            files: 0,
            display: "other".into(),
            system: false,
        }
    );
}

#[test]
fn can_tags_set_display_multiple() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();

    // add tags
    database.tag_add("test", "label").unwrap();
    database.tag_add("test", "other").unwrap();
    database.tag_add("name", "zero").unwrap();
    database.tag_add("name", "other").unwrap();

    // set display
    database
        .tag_value_display("test", "label", "Test Label")
        .unwrap();
    database
        .tag_value_display("test", "other", "Test Other")
        .unwrap();
    database
        .tag_value_display("name", "zero", "Name Zero")
        .unwrap();
    database
        .tag_value_display("name", "other", "Name Other")
        .unwrap();

    // get all tags
    let list = database.tag_list(None, None).unwrap();
    assert_eq!(list.len(), 4);
    assert_eq!(
        list[&Tag::new("test".into(), "label".into())],
        TagValueInfo {
            files: 0,
            display: "Test Label".into(),
            system: false,
        }
    );
    assert_eq!(
        list[&Tag::new("test".into(), "other".into())],
        TagValueInfo {
            files: 0,
            display: "Test Other".into(),
            system: false,
        }
    );
    assert_eq!(
        list[&Tag::new("name".into(), "zero".into())],
        TagValueInfo {
            files: 0,
            display: "Name Zero".into(),
            system: false,
        }
    );
    assert_eq!(
        list[&Tag::new("name".into(), "other".into())],
        TagValueInfo {
            files: 0,
            display: "Name Other".into(),
            system: false,
        }
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
    assert_eq!(list.len(), 2);
    assert!(list.contains_key(&Tag::new("test".into(), "label".into())));
    assert!(list.contains_key(&Tag::new("test".into(), "other".into())));
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
    assert_eq!(list.len(), 2);
    assert!(list.contains_key(&Tag::new("test".into(), "other".into())));
    assert!(list.contains_key(&Tag::new("height".into(), "other".into())));
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
    assert_eq!(list.len(), 4);
    assert!(list.contains_key(&Tag::new("foobar".into(), "label".into())));
    assert!(list.contains_key(&Tag::new("foobar".into(), "other".into())));
    assert!(list.contains_key(&Tag::new("height".into(), "zero".into())));
    assert!(list.contains_key(&Tag::new("height".into(), "other".into())));
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
    assert_eq!(list.len(), 4);
    assert!(list.contains_key(&Tag::new("test".into(), "label".into())));
    assert!(list.contains_key(&Tag::new("test".into(), "other".into())));
    assert!(list.contains_key(&Tag::new("height".into(), "zero".into())));
    assert!(list.contains_key(&Tag::new("height".into(), "new".into())));
}

#[test]
fn can_tags_delete_all() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();

    // add tags
    database.tag_add("test", "label").unwrap();
    database.tag_add("test", "other").unwrap();
    database.tag_add("value", "zero").unwrap();
    database.tag_add("value", "other").unwrap();

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
    database.tag_add("value", "zero").unwrap();
    database.tag_add("value", "other").unwrap();

    database.tag_delete(Some("test"), None).unwrap();

    // get all tags
    let list = database.tag_list(None, None).unwrap();
    assert_eq!(list.len(), 2);
    assert!(list.contains_key(&Tag::new("value".into(), "zero".into())));
    assert!(list.contains_key(&Tag::new("value".into(), "other".into())));
}

#[test]
fn can_tags_delete_by_value() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();

    // add tags
    database.tag_add("test", "label").unwrap();
    database.tag_add("test", "other").unwrap();
    database.tag_add("value", "zero").unwrap();
    database.tag_add("value", "other").unwrap();

    // delete tags by value
    database.tag_delete(None, Some("other")).unwrap();

    // get all tags
    let list = database.tag_list(None, None).unwrap();
    assert_eq!(list.len(), 2);
    assert!(list.contains_key(&Tag::new("value".into(), "zero".into())));
    assert!(list.contains_key(&Tag::new("test".into(), "label".into())));
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
    let list = database.hash_tags(&hash1, None, None).unwrap();
    assert_eq!(
        list,
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
    let list = database.hash_tags(&hash1, None, None).unwrap();
    assert_eq!(
        list,
        [
            Tag::new("name".into(), "value".into()),
            Tag::new("other".into(), "that".into())
        ]
        .into()
    );
    database
        .hash_tag_remove(&hash1, Some("name"), Some("value"))
        .unwrap();
    let list = database.hash_tags(&hash1, None, None).unwrap();
    assert_eq!(list, [Tag::new("other".into(), "that".into())].into());
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

#[test]
fn can_label_add_rect() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    let hash = Hash::new(&[0x01]);
    database.hash_add(&hash).unwrap();
    database.tag_add("name", "value").unwrap();
    database.hash_tag_add(&hash, "name", "value").unwrap();
    database
        .label_add(
            &hash,
            "name",
            "value",
            &Rectangle {
                start: Point::new(0, 0),
                end: Point::new(64, 64),
            }
            .into(),
        )
        .unwrap();
}

#[test]
fn can_label_add_seq() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    let hash = Hash::new(&[0x01]);
    database.hash_add(&hash).unwrap();
    database.tag_add("name", "value").unwrap();
    database.hash_tag_add(&hash, "name", "value").unwrap();
    database
        .label_add(
            &hash,
            "name",
            "value",
            &Sequence { start: 0, end: 55 }.into(),
        )
        .unwrap();
}

#[test]
fn can_label_remove_rect() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    let hash = Hash::new(&[0x01]);
    database.hash_add(&hash).unwrap();
    database.tag_add("name", "value").unwrap();
    database.hash_tag_add(&hash, "name", "value").unwrap();
    let label = Rectangle {
        start: Point::new(0, 0),
        end: Point::new(64, 64),
    }
    .into();
    database.label_add(&hash, "name", "value", &label).unwrap();
    database
        .label_remove(&hash, "name", "value", &label)
        .unwrap();
}

#[test]
fn can_label_remove_sequence() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    let hash = Hash::new(&[0x01]);
    database.hash_add(&hash).unwrap();
    database.tag_add("name", "value").unwrap();
    database.hash_tag_add(&hash, "name", "value").unwrap();
    let label = Sequence { start: 11, end: 99 }.into();
    database.label_add(&hash, "name", "value", &label).unwrap();
    database
        .label_remove(&hash, "name", "value", &label)
        .unwrap();
}

#[test]
fn can_label_query_empty() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    let labels = database.label_get(None, None, None, None).unwrap();
    assert_eq!(labels.len(), 0);
}

#[test]
fn can_label_get_rect() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    let hash = Hash::new(&[0x01]);
    database.hash_add(&hash).unwrap();
    database.tag_add("name", "value").unwrap();
    database.hash_tag_add(&hash, "name", "value").unwrap();
    let label = Rectangle {
        start: Point::new(0, 0),
        end: Point::new(64, 64),
    }
    .into();
    database.label_add(&hash, "name", "value", &label).unwrap();
    let labels = database.label_get(Some(&hash), None, None, None).unwrap();
    assert_eq!(
        labels,
        [(Tag::new("name".into(), "value".into()), label)].into()
    );
}

#[test]
fn can_label_get_seq() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    let hash = Hash::new(&[0x01]);
    database.hash_add(&hash).unwrap();
    database.tag_add("name", "value").unwrap();
    database.hash_tag_add(&hash, "name", "value").unwrap();
    let label = Sequence { start: 0, end: 15 }.into();
    database.label_add(&hash, "name", "value", &label).unwrap();
    let labels = database.label_get(Some(&hash), None, None, None).unwrap();
    assert_eq!(
        labels,
        [(Tag::new("name".into(), "value".into()), label)].into()
    );
}

// TODO: test label_get with more loaded data?

#[test]
fn can_get_tag_names_empty() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    database.tag_names().unwrap();
}

#[test]
fn can_get_tag_names_system() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    let names = database.tag_names().unwrap();
    assert_eq!(names["filesize"].system, true);
    assert_eq!(names["filename"].system, true);
    assert_eq!(names["directory"].system, true);
    assert_eq!(names["ancestors"].system, true);
    assert_eq!(names["media"].system, true);
    assert_eq!(names["format"].system, true);
    assert_eq!(names["duration"].system, true);
    assert_eq!(names["width"].system, true);
    assert_eq!(names["height"].system, true);
}

#[test]
fn can_get_tag_names_single() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    database.tag_add("name", "value").unwrap();
    let names = database.tag_names().unwrap();
    assert_eq!(names["name"].values, 1);
    assert_eq!(names["name"].system, false);
    assert_eq!(names["name"].display, "name");
}

#[test]
fn can_get_tag_names_multiple() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    database.tag_add("name", "value").unwrap();
    database.tag_add("name", "other").unwrap();
    database.tag_add("kind", "car").unwrap();
    let names = database.tag_names().unwrap();
    assert_eq!(names["name"].values, 2);
    assert_eq!(names["name"].system, false);
    assert_eq!(names["name"].display, "name");
    assert_eq!(names["kind"].values, 1);
    assert_eq!(names["kind"].system, false);
    assert_eq!(names["kind"].display, "kind");
}

#[test]
fn can_get_tag_name_display_custom() {
    let database = Database(Connection::open_in_memory().unwrap());
    database.migrate().unwrap();
    database.tag_add("name", "value").unwrap();
    database.tag_name_display("name", "My Name").unwrap();
    let names = database.tag_names().unwrap();
    assert_eq!(names["name"].values, 1);
    assert_eq!(names["name"].system, false);
    assert_eq!(names["name"].display, "My Name");
}
