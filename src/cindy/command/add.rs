use super::UPDATE_INTERVAL;
use crate::{
    cli::AddCommand,
    database::{Database, Handle},
    hash::{BoxHash, Hash, ReadDigester},
    Cindy, Tag,
};
use anyhow::{Context, Result};
use flume::{Receiver, Sender};
use futures::StreamExt;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{create_dir_all, hard_link, File, Metadata},
    io::{stdout, Write},
    path::{Path, PathBuf},
    time::Instant,
};
use tokio::{
    sync::mpsc::channel,
    task::{spawn_blocking, JoinHandle},
};

fn path_tags(path: &Path) -> Vec<Tag> {
    let path_tag = Tag::new("path".into(), format!("/{}", path.display()));
    let filename_tag = Tag::new(
        "filename".into(),
        path.file_name().unwrap().to_string_lossy().into_owned(),
    );
    let pathprefix_tags = path
        .ancestors()
        .skip(1)
        .map(|ancestor| Tag::new("pathprefix".into(), format!("/{}", ancestor.display())));
    [path_tag, filename_tag]
        .into_iter()
        .chain(pathprefix_tags)
        .collect()
}

fn add_file<H: Handle>(
    database: &Database<H>,
    hash: &Hash,
    tags: &[Tag],
    paths: &mut dyn Iterator<Item = &Path>,
) -> Result<()> {
    database.hash_add(&hash)?;

    for tag in tags {
        database.tag_add(tag.name(), tag.value())?;
        database.hash_tag_add(&hash, tag.name(), tag.value())?;
    }

    add_path_tags(database, hash, paths)?;

    Ok(())
}

fn add_path_tags<H: Handle>(
    database: &Database<H>,
    hash: &Hash,
    paths: &mut dyn Iterator<Item = &Path>,
) -> Result<()> {
    for path in paths {
        for tag in path_tags(&path) {
            database.tag_add(tag.name(), tag.value())?;
            database.hash_tag_add(&hash, tag.name(), tag.value())?;
        }
    }
    Ok(())
}

impl Cindy {
    pub async fn command_add(&self, command: &AddCommand) -> Result<()> {
        self.add_files(&command.paths, command.recursive).await
    }

    pub async fn add_files(&self, files: &[PathBuf], recursive: bool) -> Result<()> {
        let files = self
            .list_files(&files, recursive)
            .await
            .context("Listing files")?;
        let hashes = self.hash_files(files).await.context("Hashing files")?;
        let hashes = self
            .skip_known(hashes)
            .await
            .context("Skipping known files")?;
        let hashes = self
            .scan_metadata(hashes)
            .await
            .context("Scanning metadata")?;
        let mut database = self.database().await;
        spawn_blocking(move || {
            let transaction = database.transaction()?;
            for (hash, (metadata, paths)) in hashes.iter() {
                add_file(
                    &transaction,
                    &hash[..],
                    &metadata,
                    &mut paths.iter().map(|p| p.as_path()),
                )?;
            }
            transaction.commit()?;
            Ok(()) as Result<()>
        })
        .await??;
        Ok(())
    }

    /// Given a file, compute it's hash.
    pub fn hash_file(&self, path: &Path) -> Result<BoxHash> {
        let mut file = File::open(&self.root().join(&path))?;
        let hash = self.hasher().hash_read(&mut file)?;
        Ok(hash)
    }

    /// Given a path and a hash, add it to the data index.
    pub fn data_add(&self, file: &Path, hash: &Hash) -> Result<()> {
        let path = self.hash_path(hash);
        if !path.exists() {
            let file = self.root().join(file);
            create_dir_all(path.parent().unwrap())?;
            hard_link(file, path)?;
        }
        Ok(())
    }

    fn launch_hasher_tasks(
        &self,
        files: Receiver<(PathBuf, Metadata)>,
        hashes: Sender<(PathBuf, Metadata, BoxHash)>,
        tasks: usize,
    ) -> Vec<JoinHandle<Result<()>>> {
        (0..tasks)
            .map(|_| {
                let files = files.clone();
                let hashes = hashes.clone();
                let cindy = self.clone();
                spawn_blocking(move || {
                    for (path, metadata) in files.iter() {
                        let hash = cindy.hash_file(&path)?;
                        cindy.data_add(&path, &hash)?;
                        hashes.send((path, metadata, hash))?;
                    }
                    Ok(()) as Result<()>
                })
            })
            .collect()
    }

    fn launch_scanner_tasks(
        &self,
        files: Receiver<(BoxHash, Metadata, BTreeSet<PathBuf>)>,
        hashes: Sender<(BoxHash, Vec<Tag>, BTreeSet<PathBuf>)>,
        tasks: usize,
    ) -> Vec<JoinHandle<Result<()>>> {
        (0..tasks)
            .map(|_| {
                let files = files.clone();
                let hashes = hashes.clone();
                let cindy = self.clone();
                spawn_blocking(move || {
                    for (hash, metadata, paths) in files.iter() {
                        let filesize = Tag::new("filesize".into(), metadata.len().to_string());
                        let mut tags = vec![filesize];
                        let path = cindy.hash_path(&hash);
                        #[cfg(feature = "ffmpeg")]
                        match crate::media::media_info(&path) {
                            Ok(info) => {
                                for tag in info.tags() {
                                    tags.push(tag);
                                }
                            }
                            Err(error) => {
                                println!("Error checking media info: {error}");
                            }
                        }
                        hashes.send((hash, tags, paths))?;
                    }
                    Ok(()) as Result<()>
                })
            })
            .collect()
    }

    /// Scan file metadata.
    pub async fn scan_metadata(
        &self,
        files: BTreeMap<BoxHash, (Metadata, BTreeSet<PathBuf>)>,
    ) -> Result<BTreeMap<BoxHash, (Vec<Tag>, BTreeSet<PathBuf>)>> {
        let total_files = files.len();

        // task submitting files to queue
        let (file_sender, file_receiver) =
            flume::bounded::<(BoxHash, Metadata, BTreeSet<PathBuf>)>(1024);
        let sender = tokio::spawn(async move {
            for (hash, (metadata, paths)) in files.into_iter() {
                file_sender.send_async((hash, metadata, paths)).await?;
            }
            Ok(()) as Result<()>
        });

        // tasks to pop messages off the queue and generate hashes
        let (hash_sender, hash_receiver) =
            flume::bounded::<(BoxHash, Vec<Tag>, BTreeSet<PathBuf>)>(1024);
        let hasher_tasks = self.launch_scanner_tasks(file_receiver, hash_sender, 16);

        // start collecting hashes
        let collect = tokio::spawn(async move {
            let mut stream = hash_receiver.stream();
            let mut last_update = Instant::now();
            let mut current_files = 0;
            let mut files: BTreeMap<BoxHash, (Vec<Tag>, BTreeSet<PathBuf>)> = BTreeMap::new();
            while let Some((hash, metadata, paths)) = stream.next().await {
                current_files += 1;

                if Instant::now().duration_since(last_update) > UPDATE_INTERVAL {
                    last_update = Instant::now();
                    print!("\r\x1B[2Kscanning {current_files}/{total_files} files)");
                    stdout().flush().unwrap();
                }

                files.insert(hash, (metadata, paths));
            }

            println!("\r\x1B[2Kscanning {current_files}/{total_files} files");
            Ok(files) as Result<BTreeMap<BoxHash, (Vec<Tag>, BTreeSet<PathBuf>)>>
        });

        // await for futures
        for task in hasher_tasks.into_iter() {
            task.await??;
        }
        sender.await??;
        Ok(collect.await??)
    }

    /// Skip known files
    pub async fn skip_known(
        &self,
        mut files: BTreeMap<BoxHash, (Metadata, BTreeSet<PathBuf>)>,
    ) -> Result<BTreeMap<BoxHash, (Metadata, BTreeSet<PathBuf>)>> {
        let mut database = self.database().await;
        spawn_blocking(move || {
            let mut last_update = Instant::now();
            let total_files = files.len();
            let mut current_files = 0;

            // check for existing files
            let transaction = database.transaction()?;
            let mut exists = BTreeSet::new();
            for (hash, (_metadata, paths)) in &files {
                current_files += 1;
                if Instant::now().duration_since(last_update) > UPDATE_INTERVAL {
                    last_update = Instant::now();
                    print!("\r\x1B[2Kdeduplicating {current_files}/{total_files} files");
                    stdout().flush().unwrap();
                }

                // if a file already exists, just save the paths
                if transaction.hash_exists(&hash)? {
                    exists.insert(hash.clone());
                    add_path_tags(&transaction, hash, &mut paths.iter().map(PathBuf::as_path))?;
                }
            }

            transaction.commit()?;

            // remove files that already exists
            for hash in exists {
                files.remove(&hash);
            }

            println!("\r\x1B[2Kdeduplicating {current_files}/{total_files} files");

            Ok(files) as Result<_>
        })
        .await?
    }

    /// Hash files.
    pub async fn hash_files(
        &self,
        files: BTreeMap<PathBuf, Metadata>,
    ) -> Result<BTreeMap<BoxHash, (Metadata, BTreeSet<PathBuf>)>> {
        let total_files = files.len();
        let total_bytes: u64 = files.iter().map(|(_, metadata)| metadata.len()).sum();

        // task submitting files to queue
        let (file_sender, file_receiver) = flume::bounded::<(PathBuf, Metadata)>(1024);
        let sender = tokio::spawn(async move {
            for (file, metadata) in files.into_iter() {
                file_sender.send_async((file, metadata)).await?;
            }
            Ok(()) as Result<()>
        });

        // tasks to pop messages off the queue and generate hashes
        let (hash_sender, hash_receiver) = flume::bounded::<(PathBuf, Metadata, BoxHash)>(1024);
        let hasher_tasks = self.launch_hasher_tasks(file_receiver, hash_sender, 16);

        // start collecting hashes
        let collect = tokio::spawn(async move {
            let mut stream = hash_receiver.stream();
            let mut last_update = Instant::now();
            let mut current_files = 0;
            let mut current_bytes = 0;
            let mut files: BTreeMap<BoxHash, (Metadata, BTreeSet<PathBuf>)> = BTreeMap::new();
            while let Some((path, metadata, hash)) = stream.next().await {
                current_files += 1;
                current_bytes += metadata.len();

                if Instant::now().duration_since(last_update) > UPDATE_INTERVAL {
                    last_update = Instant::now();
                    print!("\r\x1B[2Khashing {current_files}/{total_files} files ({current_bytes}/{total_bytes} bytes)");
                    stdout().flush().unwrap();
                }

                files
                    .entry(hash)
                    .or_insert_with(move || (metadata, BTreeSet::new()))
                    .1
                    .insert(path);
            }

            println!("\r\x1B[2Khashing {current_files}/{total_files} files ({current_bytes}/{total_bytes} bytes)");
            Ok(files) as Result<BTreeMap<BoxHash, (Metadata, BTreeSet<PathBuf>)>>
        });

        // await for futures
        for task in hasher_tasks.into_iter() {
            task.await??;
        }
        sender.await??;
        Ok(collect.await??)
    }

    /// List files (recursively)
    pub async fn list_files(
        &self,
        paths: &[PathBuf],
        recursive: bool,
    ) -> Result<BTreeMap<PathBuf, Metadata>> {
        let (sender, mut receiver) = channel::<(PathBuf, Metadata)>(1024);
        let files = tokio::spawn(async move {
            let mut files: BTreeMap<PathBuf, Metadata> = BTreeMap::new();
            let mut last_update = Instant::now();
            let mut bytes = 0;
            while let Some((path, metadata)) = receiver.recv().await {
                if Instant::now().duration_since(last_update) > UPDATE_INTERVAL {
                    last_update = Instant::now();
                    print!("\r\x1B[2Klisting {} files ({bytes} bytes)", files.len());
                    stdout().flush().unwrap();
                }

                let file_len = metadata.len();
                if files.insert(path, metadata).is_none() {
                    bytes += file_len;
                }
            }

            println!("\rlisting {} files ({bytes} bytes)", files.len());
            files
        });

        for path in paths {
            let path = std::fs::canonicalize(&path)?;
            if recursive {
                let sender = sender.clone();
                let root = self.root().to_path_buf();
                let cindy = self.clone();
                spawn_blocking(move || {
                    // make sure we don't recurse into our own data or thumbs paths
                    let cindy_folder = cindy.cindy_folder();
                    let filter = |path: &Path| {
                        if path == cindy_folder {
                            return false;
                        }

                        true
                    };

                    // scan files recursively
                    for result in scan_files(&path, &filter) {
                        let (path, metadata) = result?;
                        let path = path.strip_prefix(&root)?.to_path_buf();
                        let sender = sender.clone();
                        sender.blocking_send((path, metadata))?;
                    }

                    Ok(()) as Result<()>
                })
                .await??;
            } else {
                let metadata = tokio::fs::metadata(&path).await?;
                let path = path.strip_prefix(self.root())?.to_path_buf();
                sender.send((path, metadata)).await?;
            }
        }
        drop(sender);

        Ok(files.await?)
    }
}

fn scan_files<'a>(
    path: &Path,
    filter: &'a dyn Fn(&Path) -> bool,
) -> impl Iterator<Item = Result<(PathBuf, Metadata)>> + 'a {
    let files = std::fs::read_dir(&path).unwrap();
    files
        .map(move |entry| {
            let entry = entry?;
            let path = entry.path().to_path_buf();
            let metadata = entry.metadata()?;
            Ok((path, metadata))
        })
        .map(|result| {
            let result: Box<dyn Iterator<Item = Result<(PathBuf, Metadata)>> + 'a> = match result {
                Ok((path, metadata)) if metadata.is_dir() && filter(&path) => {
                    Box::new(scan_files(&path, filter))
                }
                Ok((_, metadata)) if !metadata.is_file() => Box::new(std::iter::empty()),
                other => Box::new(std::iter::once(other)),
            };
            result
        })
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir, write};
    use tempfile::tempdir;

    #[test]
    fn test_path_tags() {
        let tags = path_tags(Path::new("images/vacation/boat.jpg"));
        assert_eq!(
            tags,
            [
                Tag::new("path".into(), "/images/vacation/boat.jpg".into()),
                Tag::new("filename".into(), "boat.jpg".into()),
                Tag::new("pathprefix".into(), "/images/vacation".into()),
                Tag::new("pathprefix".into(), "/images".into()),
                Tag::new("pathprefix".into(), "/".into()),
            ]
        );
    }

    #[test]
    fn can_scan_files() {
        let dir = tempdir().unwrap();
        let files = scan_files(&dir.path(), &|_| true).count();
        assert_eq!(files, 0);
    }

    #[test]
    fn can_scan_files_many2() {
        let dir = tempdir().unwrap();
        write(dir.path().join("file1.txt"), "hello").unwrap();
        write(dir.path().join("file2.txt"), "world").unwrap();
        write(dir.path().join("file3.txt"), "test").unwrap();
        let files: BTreeMap<_, _> = scan_files(&dir.path(), &|_| true)
            .map(|result| {
                result.map(|(path, metadata)| {
                    (
                        path.strip_prefix(dir.path()).unwrap().to_path_buf(),
                        metadata,
                    )
                })
            })
            .collect::<Result<BTreeMap<_, _>, _>>()
            .unwrap();
        assert_eq!(files.len(), 3);
        assert_eq!(files.get(Path::new("file1.txt")).unwrap().len(), 5);
        assert_eq!(files.get(Path::new("file2.txt")).unwrap().len(), 5);
        assert_eq!(files.get(Path::new("file3.txt")).unwrap().len(), 4);
    }

    #[test]
    fn can_scan_files_folder() {
        let dir = tempdir().unwrap();
        // create folders
        create_dir(dir.path().join("folder")).unwrap();
        create_dir(dir.path().join("other")).unwrap();
        create_dir(dir.path().join("hidden")).unwrap();

        // create files
        write(dir.path().join("folder").join("file1.txt"), "hello").unwrap();
        write(dir.path().join("folder").join("file2.txt"), "world").unwrap();
        write(dir.path().join("folder").join("file3.txt"), "test").unwrap();
        write(dir.path().join("other").join("taxes.txt"), "expensive").unwrap();
        write(dir.path().join("hidden").join("secrets.txt"), "shh").unwrap();

        // iterate
        let files: BTreeMap<_, _> = scan_files(&dir.path(), &|path| {
            path.strip_prefix(dir.path())
                .map(|path| path != Path::new("hidden"))
                .unwrap_or(true)
        })
        .map(|result| {
            result.map(|(path, metadata)| {
                (
                    path.strip_prefix(dir.path()).unwrap().to_path_buf(),
                    metadata,
                )
            })
        })
        .collect::<Result<BTreeMap<_, _>, _>>()
        .unwrap();
        assert_eq!(files.len(), 4);
        assert_eq!(files.get(Path::new("folder/file1.txt")).unwrap().len(), 5);
        assert_eq!(files.get(Path::new("folder/file2.txt")).unwrap().len(), 5);
        assert_eq!(files.get(Path::new("folder/file3.txt")).unwrap().len(), 4);
    }
}
