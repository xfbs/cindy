use crate::{
    hash::{BoxHash, Hash},
    tag::Tag,
};
use anyhow::Result;
use rusqlite::{Connection, Error, Transaction};
use std::{collections::BTreeSet, ops::Deref};

pub const SQLITE_SCHEMA: &str = include_str!("database/schema.sql");

mod handlers;
#[cfg(test)]
mod tests;

/// Handle to database.
pub trait Handle {
    /// Provide a connection to the database.
    fn connection(&self) -> &Connection;
}

impl Handle for Connection {
    fn connection(&self) -> &Connection {
        &self
    }
}

impl<'a> Handle for Transaction<'a> {
    fn connection(&self) -> &Connection {
        self.deref()
    }
}

/// Database wrapper.
///
/// Dereferences to a [`Connection`], but provides additional methods.
#[derive(Debug)]
pub struct Database<T = Connection>(T);

impl Database<Connection> {
    pub fn transaction(&mut self) -> Result<Database<Transaction<'_>>, Error> {
        self.0.transaction().map(Database)
    }
}

impl Database<Transaction<'_>> {
    pub fn commit(self) -> Result<(), Error> {
        self.0.commit()
    }

    pub fn abort(self) -> Result<()> {
        Ok(())
    }
}

impl<T: Handle> From<T> for Database<T> {
    fn from(backend: T) -> Self {
        Self(backend)
    }
}

impl<T: Handle> Deref for Database<T> {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        self.0.connection()
    }
}

#[test]
fn test_transaction() {
    let mut connection: Database = Connection::open_in_memory().unwrap().into();
    format!("{connection:?}");
    connection.execute("PRAGMA foreign_keys=ON", ()).unwrap();
    let transaction = connection.transaction().unwrap();
    transaction.abort().unwrap();
    let transaction = connection.transaction().unwrap();
    transaction.execute("PRAGMA foreign_keys=ON", ()).unwrap();
    transaction.commit().unwrap();
}
