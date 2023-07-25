use super::*;
use crate::tag::TagPredicate;
use rusqlite::ToSql;

impl<T: Handle> Database<T> {
    /// Add hash to database.
    pub fn hash_add(&self, hash: &Hash) -> Result<()> {
        let mut query = self.prepare_cached("INSERT OR IGNORE INTO files(hash) VALUES (?)")?;
        query.execute([hash])?;
        Ok(())
    }

    /// Remove file hash from database, including all tags.
    pub fn hash_remove(&self, hash: &Hash) -> Result<()> {
        let mut query = self.prepare_cached("DELETE FROM files WHERE hash = ?")?;
        query.execute([hash])?;
        Ok(())
    }

    /// Check if a hash exists.
    pub fn hash_exists(&self, hash: &Hash) -> Result<bool> {
        let mut query = self.prepare_cached("SELECT * FROM files WHERE hash = ?")?;
        let mut rows = query.query([hash])?;
        Ok(rows.next()?.is_some())
    }

    pub fn hash_tags(&self, hash: &Hash) -> Result<BTreeSet<Tag>> {
        let mut query = self.prepare_cached(
            "SELECT name, value
            FROM file_tags
            WHERE hash = ?",
        )?;
        let rows = query.query([&hash])?;
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
        query.execute((file, &tag, &value))?;
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
        query.execute((file, &tag, &value))?;
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
        rows.mapped(|row| Ok(row.get::<_, Vec<u8>>("hash")?.into()))
            .collect::<Result<BTreeSet<BoxHash>, _>>()
            .map_err(Into::into)
    }

    /// Run migrations on database.
    pub fn migrate(&self) -> Result<()> {
        self.execute_batch(SQLITE_SCHEMA)?;
        Ok(())
    }
}
