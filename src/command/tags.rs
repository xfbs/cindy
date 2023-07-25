use crate::{
    cli::{TagsCommand, TagsCreateCommand, TagsDeleteCommand, TagsListCommand, TagsRenameCommand},
    tag::Tag,
    Cindy,
};
use anyhow::Result;
use std::collections::BTreeSet;

impl Cindy {
    pub async fn command_tags(&self, command: &TagsCommand) -> Result<()> {
        match command {
            TagsCommand::List(command) => self.command_tags_list(command).await,
            TagsCommand::Create(command) => self.command_tags_create(command).await,
            _ => Ok(()),
        }
    }

    pub async fn command_tags_delete(&self, _command: &TagsDeleteCommand) -> Result<()> {
        todo!()
    }

    pub async fn command_tags_rename(&self, _command: &TagsRenameCommand) -> Result<()> {
        todo!()
    }

    pub async fn command_tags_create(&self, command: &TagsCreateCommand) -> Result<()> {
        let database = self.database().await;
        let command = command.clone();
        tokio::task::spawn_blocking(move || {
            for tag in command.tags {
                database.tag_add(tag.name(), tag.value())?;
            }
            Ok(()) as Result<()>
        })
        .await??;
        Ok(())
    }

    pub async fn command_tags_list(&self, command: &TagsListCommand) -> Result<()> {
        let database = self.database().await;
        let command = command.clone();
        tokio::task::spawn_blocking(move || {
            let tags = if command.tags.is_empty() {
                database.tag_list(None, None)?
            } else {
                let results: Vec<BTreeSet<Tag>> = command
                    .tags
                    .iter()
                    .map(|filter| database.tag_list(filter.name(), filter.value()))
                    .collect::<Result<Vec<BTreeSet<Tag>>>>()?;
                results.into_iter().flat_map(|i| i.into_iter()).collect()
            };
            for tag in tags.iter() {
                println!("{tag}");
            }
            Ok(()) as Result<()>
        })
        .await??;
        Ok(())
    }
}
