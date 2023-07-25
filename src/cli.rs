use crate::tag::{Tag, TagFilter, TagPredicate};
use clap::Parser;
use std::{net::SocketAddr, path::PathBuf};

/// Global options.
#[derive(Parser, Clone, Debug, Default)]
pub struct GlobalOptions {
    /// Configuration file.
    ///
    /// By default, Cindy will automatically discover this by walking up the current working
    /// directory until it finds a cindy.toml file.
    #[clap(long, env, global = true)]
    pub config: Option<PathBuf>,

    /// Turn on verbose output.
    #[clap(long, short, global = true)]
    pub verbose: bool,

    /// Turn on verbose output.
    #[clap(long, short = 'j', global = true)]
    pub threads: Option<u64>,
}

/// Cindy command-line options.
#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// Global options, shared by all subcommands.
    #[clap(flatten)]
    pub global: GlobalOptions,

    /// Subcommand to run.
    #[clap(subcommand)]
    pub command: Command,
}

/// Initialize new Cindy project.
#[derive(Parser, Clone, Debug)]
pub struct InitCommand {
    #[clap(default_value = ".")]
    pub path: PathBuf,
}

/// Serve Cindy web user interface.
#[derive(Parser, Clone, Debug)]
pub struct ServeCommand {
    #[clap(short, long, default_value = "127.0.0.1:8000")]
    pub listen: SocketAddr,
}

#[derive(Parser, Clone, Debug)]
pub struct AddCommand {
    /// Add files recursively.
    #[clap(long, short)]
    pub recursive: bool,

    #[clap(default_value = ".")]
    pub paths: Vec<PathBuf>,
}

#[derive(Parser, Clone, Debug)]
pub struct QueryCommand {
    /// Show tags of each media.
    #[clap(long)]
    pub tags: bool,

    /// Show paths instead of the hashes.
    #[clap(long)]
    pub paths: bool,

    pub filters: Vec<TagPredicate<'static>>,
}

#[derive(Parser, Clone, Debug)]
pub struct ListCommand {
    #[clap(default_value = ".")]
    pub path: PathBuf,

    #[clap(long, short)]
    pub recursive: bool,
}

#[derive(Parser, Clone, Debug)]
pub struct EditCommand {
    /// Tag to add.
    #[clap(long, short)]
    pub add: Vec<Tag>,

    /// Tag to remove.
    #[clap(long, short = 'd')]
    pub remove: Vec<Tag>,

    /// Apply to files recursively.
    #[clap(long, short)]
    pub recursive: bool,

    /// List of files to apply it to.
    pub files: Vec<PathBuf>,
}

#[derive(Parser, Clone, Debug)]
pub struct RemoveCommand {
    #[clap(long, short)]
    pub recursive: bool,

    #[clap(default_value = ".")]
    pub paths: Vec<PathBuf>,
}

#[derive(Parser, Clone, Debug)]
pub struct TagsCreateCommand {
    pub tags: Vec<Tag>,
}

#[derive(Parser, Clone, Debug)]
pub struct TagsDeleteCommand {
    pub tags: Vec<TagFilter<'static>>,

    /// Force deleting a tag if it is still in use.
    #[clap(short, long)]
    pub force: bool,
}

#[derive(Parser, Clone, Debug)]
pub struct TagsRenameCommand {
    pub old: TagFilter<'static>,
    pub new: TagFilter<'static>,
}

#[derive(Parser, Clone, Debug)]
pub struct TagsListCommand {
    pub tags: Vec<TagFilter<'static>>,
}

#[derive(Parser, Clone, Debug)]
pub enum TagsCommand {
    /// Create a new tag.
    Create(TagsCreateCommand),
    /// Delete a tag.
    Delete(TagsDeleteCommand),
    /// Rename a tag.
    Rename(TagsRenameCommand),
    /// List tags.
    List(TagsListCommand),
}

#[derive(Parser, Clone, Debug)]
pub enum Command {
    /// Initialize new Cindy project.
    Init(InitCommand),
    /// Add files to the Cindy index.
    Add(AddCommand),
    /// Remove files from the Cindy index.
    #[clap(alias = "rm")]
    Remove(RemoveCommand),
    /// Query files in the Cindy project.
    Query(QueryCommand),
    /// List files
    #[clap(alias = "ls")]
    List(ListCommand),
    /// Manage tags for files.
    Edit(EditCommand),
    /// Manage tags
    #[clap(subcommand)]
    Tags(TagsCommand),
    /// Serve Cindy UI.
    #[cfg(feature = "server")]
    #[clap(alias = "server")]
    Serve(ServeCommand),
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn arb_tag_filter() -> impl Strategy<Value = TagFilter<'static>> {
        prop_oneof![
            Just(TagFilter::new::<&str>(None, None)),
            "[a-z]{4}".prop_map(|string| TagFilter::new::<String>(Some(string), None)),
            "[a-z]{4}".prop_map(|string| TagFilter::new::<String>(None, Some(string))),
            ("[a-z]{4}", "[a-z]{4}")
                .prop_map(|(name, value)| TagFilter::new::<String>(Some(name), Some(value))),
        ]
        .boxed()
    }

    fn arb_tag() -> impl Strategy<Value = Tag> {
        ("[a-z]{4}", "[a-z]{4}").prop_map(|(name, value)| Tag::new(name, value))
    }

    fn arb_options() -> impl Strategy<Value = Options> {
        arb_command().prop_map(|command| Options {
            global: Default::default(),
            command,
        })
    }

    fn arb_tag_predicate() -> impl Strategy<Value = TagPredicate<'static>> {
        prop_oneof![
            arb_tag_filter().prop_map(TagPredicate::Exists),
            arb_tag_filter().prop_map(TagPredicate::Missing),
        ]
    }

    fn arb_path_buf() -> impl Strategy<Value = PathBuf> {
        prop_oneof![
            Just(PathBuf::from(".")),
            "[a-z]{4}".prop_map(|seg| PathBuf::from(seg)),
            ("[a-z]{4}", "[a-z]{4}").prop_map(|(seg1, seg2)| PathBuf::from(seg1).join(seg2)),
            ("[a-z]{4}", "[a-z]{4}", "[a-z]{4}")
                .prop_map(|(seg1, seg2, seg3)| PathBuf::from(seg1).join(seg2).join(seg3)),
        ]
    }

    fn arb_path_buf_list_or_pwd() -> impl Strategy<Value = Vec<PathBuf>> {
        prop_oneof![
            Just(vec![PathBuf::from(".")]),
            prop::collection::vec(arb_path_buf(), 1..10),
        ]
    }

    prop_compose! {
        fn arb_init_command()(path in arb_path_buf()) -> InitCommand {
            InitCommand {
                path,
            }
        }
    }

    prop_compose! {
        fn arb_list_command()(recursive in prop::bool::ANY, path in arb_path_buf()) -> ListCommand {
            ListCommand {
                path,
                recursive,
            }
        }
    }

    prop_compose! {
        fn arb_query_command()(filters in prop::collection::vec(arb_tag_predicate(), 0..10)) -> QueryCommand {
            QueryCommand {
                filters,
                tags: false,
                paths: false,
            }
        }
    }

    prop_compose! {
        fn arb_edit_command()(
            files in prop::collection::vec(arb_path_buf(), 1..10),
            add in prop::collection::vec(arb_tag(), 0..5),
            remove in prop::collection::vec(arb_tag(), 0..5),
            recursive in prop::bool::ANY
        ) -> EditCommand {
            EditCommand {
                recursive,
                add,
                remove,
                files,
            }
        }
    }

    prop_compose! {
        fn arb_add_command()(recursive in prop::bool::ANY, paths in arb_path_buf_list_or_pwd()) -> AddCommand {
            AddCommand {
                recursive,
                paths,
            }
        }
    }

    prop_compose! {
        fn arb_remove_command()(recursive in prop::bool::ANY, paths in arb_path_buf_list_or_pwd()) -> RemoveCommand {
            RemoveCommand {
                recursive,
                paths,
            }
        }
    }

    fn arb_command() -> impl Strategy<Value = Command> {
        prop_oneof![
            arb_init_command().prop_map(Command::Init),
            arb_add_command().prop_map(Command::Add),
            arb_remove_command().prop_map(Command::Remove),
            arb_query_command().prop_map(Command::Query),
            arb_list_command().prop_map(Command::List),
            arb_edit_command().prop_map(Command::Edit),
        ]
    }

    proptest! {
        #[test]
        fn command_clone(command in arb_command()) {
            let _command_clone = command.clone();
        }

        #[test]
        fn command_debug(command in arb_command()) {
            let _command_debug = format!("{command:?}");
        }

        #[test]
        fn options_clone(options in arb_options()) {
            let _options_clone = options.clone();
        }

        #[test]
        fn options_debug(options in arb_options()) {
            let _options_debug = format!("{options:?}");
        }
    }

    #[test]
    fn cli_examples() {
        // initialize new project
        Options::try_parse_from(&["cindy", "init"]).unwrap();
        Options::try_parse_from(&["cindy", "init", "folder"]).unwrap();

        // add files (recursively)
        Options::try_parse_from(&["cindy", "add", "file1", "file2"]).unwrap();
        Options::try_parse_from(&["cindy", "add", "-r", "folder"]).unwrap();

        // remove files (recursively)
        Options::try_parse_from(&["cindy", "remove", "file1", "file2"]).unwrap();
        Options::try_parse_from(&["cindy", "remove", "-r", "folder"]).unwrap();
        Options::try_parse_from(&["cindy", "rm", "file1", "file2"]).unwrap();
        Options::try_parse_from(&["cindy", "rm", "-r", "folder"]).unwrap();

        // query files
        Options::try_parse_from(&["cindy", "query", "name:value", "name:other"]).unwrap();
        Options::try_parse_from(&["cindy", "query", "name:value", "!name:other"]).unwrap();

        // list files
        Options::try_parse_from(&["cindy", "list"]).unwrap();
        Options::try_parse_from(&["cindy", "list", "folder"]).unwrap();
        Options::try_parse_from(&["cindy", "list", "-r"]).unwrap();
        Options::try_parse_from(&["cindy", "list", "-r", "folder"]).unwrap();
        Options::try_parse_from(&["cindy", "ls"]).unwrap();
        Options::try_parse_from(&["cindy", "ls", "folder"]).unwrap();
        Options::try_parse_from(&["cindy", "ls", "-r"]).unwrap();
        Options::try_parse_from(&["cindy", "ls", "-r", "folder"]).unwrap();

        Options::try_parse_from(&["cindy", "edit", "file", "--add", "name:value"]).unwrap();
        Options::try_parse_from(&["cindy", "edit", "file", "--remove", "name:value"]).unwrap();

        Options::try_parse_from(&["cindy", "tags", "create", "name:value"]).unwrap();
        Options::try_parse_from(&["cindy", "tags", "delete", "name:value"]).unwrap();
        Options::try_parse_from(&["cindy", "tags", "delete", "--force", "name:value"]).unwrap();
        Options::try_parse_from(&["cindy", "tags", "list"]).unwrap();
        Options::try_parse_from(&["cindy", "tags", "list", "name:*"]).unwrap();
        Options::try_parse_from(&["cindy", "tags", "rename", "name:value", "name:other"]).unwrap();
    }
}
