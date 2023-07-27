use super::*;
use crate::tag::TagPredicate;
use cindy_common::{Label, LabelKind, Point, Rectangle, Sequence};
use rusqlite::ToSql;

// Database interactions return Sqlite errors.
type Result<T, E = rusqlite::Error> = std::result::Result<T, E>;

impl<T: Handle> Database<T> {
    /// Add hash to database.
    pub fn hash_add(&self, hash: &Hash) -> Result<()> {
        let mut query = self.prepare_cached("INSERT OR IGNORE INTO files(hash) VALUES (?)")?;
        query.execute([hash.as_ref()])?;
        Ok(())
    }

    /// Remove file hash from database, including all tags.
    pub fn hash_remove(&self, hash: &Hash) -> Result<()> {
        let mut query = self.prepare_cached("DELETE FROM files WHERE hash = ?")?;
        query.execute([hash.as_ref()])?;
        Ok(())
    }

    /// Check if a hash exists.
    pub fn hash_exists(&self, hash: &Hash) -> Result<bool> {
        let mut query = self.prepare_cached("SELECT * FROM files WHERE hash = ?")?;
        let mut rows = query.query([hash.as_ref()])?;
        Ok(rows.next()?.is_some())
    }

    pub fn hash_tags(
        &self,
        hash: &Hash,
        name: Option<&str>,
        value: Option<&str>,
    ) -> Result<BTreeSet<Tag>> {
        let mut query = self.prepare_cached(
            "SELECT name, value
            FROM file_tags
            WHERE hash = ?
            AND coalesce(name = ?, true)
            AND coalesce(value = ?, true)",
        )?;
        let rows = query.query((hash.as_ref(), name, value))?;
        rows.mapped(|row| Ok(Tag::new(row.get("name")?, row.get("value")?)))
            .collect::<Result<BTreeSet<Tag>, _>>()
            .map_err(Into::into)
    }

    /// Add tag to database.
    pub fn tag_add(&self, tag: &str, value: &str) -> Result<()> {
        let mut query =
            self.prepare_cached("INSERT OR IGNORE INTO tags(name, value) VALUES (?, ?)")?;
        query.execute([&tag, &value])?;
        Ok(())
    }

    /// List tags in database.
    pub fn tag_list(&self, name: Option<&str>, value: Option<&str>) -> Result<BTreeSet<Tag>> {
        let mut query = self.prepare_cached(
            "SELECT name, value
            FROM tags
            WHERE coalesce(name = ?, true)
            AND coalesce(value = ?, true)",
        )?;
        let rows = query.query([&name, &value])?;
        rows.mapped(|row| Ok(Tag::new(row.get("name")?, row.get("value")?)))
            .collect::<Result<BTreeSet<Tag>, _>>()
            .map_err(Into::into)
    }

    /// Rename tag name.
    pub fn tag_name_rename(&self, old: &str, new: &str) -> Result<()> {
        let mut query = self.prepare_cached("UPDATE tag_names SET name = ? WHERE name = ?")?;
        query.execute([&new, &old])?;
        Ok(())
    }

    /// Rename tag value.
    pub fn tag_value_rename(&self, name: &str, old: &str, new: &str) -> Result<()> {
        let mut query = self.prepare_cached(
            "UPDATE tag_values
            SET value = ?
            WHERE value = ?
            AND tag_id = (SELECT id FROM tag_names WHERE name = ?)",
        )?;
        query.execute((&new, &old, name))?;
        Ok(())
    }

    /// Delete tags.
    pub fn tag_delete(&self, name: Option<&str>, value: Option<&str>) -> Result<()> {
        let mut query = self.prepare_cached(
            "DELETE FROM tags
            WHERE coalesce(name = ?, true)
            AND coalesce(value = ?, true)",
        )?;
        query.execute([&name, &value])?;
        Ok(())
    }

    /// Add tag to file
    pub fn hash_tag_add(&self, file: &Hash, tag: &str, value: &str) -> Result<()> {
        let mut query = self.prepare_cached(
            "INSERT OR IGNORE INTO file_tags(hash, name, value)
            VALUES (?, ?, ?)",
        )?;
        query.execute((file.as_ref(), &tag, &value))?;
        Ok(())
    }

    /// Remove tag from file
    pub fn hash_tag_remove(
        &self,
        file: &Hash,
        tag: Option<&str>,
        value: Option<&str>,
    ) -> Result<()> {
        let mut query = self.prepare_cached(
            "DELETE FROM file_tags
            WHERE coalesce(hash = ?, true)
            AND coalesce(name = ?, true)
            AND coalesce(value = ?, true)",
        )?;
        query.execute((file.as_ref(), &tag, &value))?;
        Ok(())
    }

    pub fn hash_query(
        &self,
        query: &mut dyn Iterator<Item = &TagPredicate<'_>>,
    ) -> Result<BTreeSet<BoxHash>> {
        let mut params: Vec<Option<&str>> = vec![];
        let mut segments = vec![];
        for predicate in query {
            let filter = predicate.filter();
            let segment = "
                (EXISTS (SELECT file_id FROM file_tags
                    WHERE files.id = file_tags.file_id
                    AND coalesce(name = ?, true)
                    AND coalesce(value = ?, true)
                    ))
            "
            .to_string();
            let segment = match predicate {
                TagPredicate::Missing(_) => format!("(NOT {segment})"),
                _other => segment,
            };
            params.push(filter.name());
            params.push(filter.value());
            segments.push(segment);
        }
        let query_string = match segments.len() {
            0 => "SELECT hash FROM files".into(),
            _ => format!("SELECT hash FROM files WHERE {}", segments.join(" AND ")),
        };
        let mut query = self.prepare(&query_string)?;
        let params: Vec<&dyn ToSql> = params.iter().map(|v| v as &dyn ToSql).collect();
        let rows = query.query(&params[..])?;
        rows.mapped(|row| Ok(Box::<[u8]>::from(row.get::<_, Vec<u8>>("hash")?).into()))
            .collect::<Result<BTreeSet<BoxHash>, _>>()
            .map_err(Into::into)
    }

    /// Add a label to a tagged file.
    pub fn label_add(&self, file: &Hash, name: &str, value: &str, label: &Label) -> Result<()> {
        match label {
            Label::Rectangle(rect) => self.label_add_rect(file, name, value, rect),
            Label::Sequence(seq) => self.label_add_seq(file, name, value, seq),
        }
    }

    fn label_add_rect(&self, file: &Hash, name: &str, value: &str, rect: &Rectangle) -> Result<()> {
        let mut query = self.prepare_cached(
            "INSERT OR IGNORE INTO label_rectangles(file_tag_value_id, x1, y1, x2, y2)
            VALUES (
                (SELECT id FROM file_tags WHERE hash = ? AND name = ? AND value = ?),
                ?, ?, ?, ?
            )
        ",
        )?;
        query.execute((
            file.as_ref(),
            name,
            value,
            rect.start.x,
            rect.start.y,
            rect.end.x,
            rect.end.y,
        ))?;
        Ok(())
    }

    fn label_add_seq(&self, file: &Hash, name: &str, value: &str, seq: &Sequence) -> Result<()> {
        let mut query = self.prepare_cached(
            "
            INSERT OR IGNORE INTO label_sequences(file_tag_value_id, t1, t2)
            VALUES (
                (SELECT id FROM file_tags WHERE hash = ? AND name = ? AND value = ?),
                ?, ?
            )
        ",
        )?;
        query.execute((file.as_ref(), name, value, seq.start, seq.end))?;
        Ok(())
    }

    /// Add a label to a tagged file.
    pub fn label_remove(&self, file: &Hash, name: &str, value: &str, label: &Label) -> Result<()> {
        match label {
            Label::Rectangle(rect) => self.label_remove_rect(file, name, value, rect),
            Label::Sequence(seq) => self.label_remove_seq(file, name, value, seq),
        }
    }

    fn label_remove_rect(
        &self,
        file: &Hash,
        name: &str,
        value: &str,
        rect: &Rectangle,
    ) -> Result<()> {
        let mut query = self.prepare_cached(
            "DELETE FROM label_rectangles
            WHERE file_tag_value_id = (SELECT id FROM file_tags WHERE hash = ? AND name = ? AND value = ?)
            AND x1 = ?
            AND y1 = ?
            AND x2 = ?
            AND y2 = ?"
        )?;
        query.execute((
            file.as_ref(),
            name,
            value,
            rect.start.x,
            rect.start.y,
            rect.end.x,
            rect.end.y,
        ))?;
        Ok(())
    }

    fn label_remove_seq(&self, file: &Hash, name: &str, value: &str, seq: &Sequence) -> Result<()> {
        let mut query = self.prepare_cached(
            "DELETE FROM label_sequences
            WHERE file_tag_value_id = (SELECT id FROM file_tags WHERE hash = ? AND name = ? AND value = ?)
            AND t1 = ?
            AND t2 = ?"
        )?;
        query.execute((file.as_ref(), name, value, seq.start, seq.end))?;
        Ok(())
    }

    // TODO: rename this to label_query and create label_get which takes a fixed hash, name and
    // value but only returns Labels?
    pub fn label_get(
        &self,
        file: Option<&Hash>,
        name: Option<&str>,
        value: Option<&str>,
        kind: Option<LabelKind>,
    ) -> Result<BTreeSet<(Tag, Label)>> {
        let mut query = self.prepare_cached(
            "SELECT *
            FROM file_labels
            WHERE coalesce(hash = ?, true)
            AND coalesce(name = ?, true)
            AND coalesce(value = ?, true)
            AND coalesce(kind = ?, true)",
        )?;
        let rows = query.query((
            file.map(|f| f.as_ref()),
            name,
            value,
            kind.map(|k| k.name()),
        ))?;
        rows.mapped(|row| {
            let tag = Tag::new(row.get("name")?, row.get("value")?);
            let label = match row.get::<_, String>("kind")? {
                kind if kind == LabelKind::Rectangle.name() => Rectangle {
                    start: Point::new(row.get("x1")?, row.get("y1")?),
                    end: Point::new(row.get("x2")?, row.get("y2")?),
                }
                .into(),
                kind if kind == LabelKind::Sequence.name() => Sequence {
                    start: row.get("t1")?,
                    end: row.get("t2")?,
                }
                .into(),
                _ => unreachable!("encountered unknown label kind"),
            };
            Ok((tag, label))
        })
        .collect::<Result<BTreeSet<(Tag, Label)>, _>>()
        .map_err(Into::into)
    }

    /// Run migrations on database.
    pub fn migrate(&self) -> Result<()> {
        self.execute_batch(SQLITE_SCHEMA)?;
        Ok(())
    }
}
