use anyhow::Context;
use bytes::Bytes;
use futures_core::{Future, Stream};
use std::{marker::PhantomData, path::PathBuf};
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
};
use tokio_stream::StreamExt;

pub trait Key {
    type Serialization: AsRef<str>;

    fn serialize(&self) -> Self::Serialization;
}

pub struct FileCache<K, Fetch, FetchReturn, FetchOutput>
where
    K: Key + Copy,
    Fetch: Fn(K) -> FetchReturn,
    FetchReturn: Future<Output = anyhow::Result<FetchOutput>>,
    FetchOutput: Stream<Item = anyhow::Result<Bytes>>,
{
    key: PhantomData<K>,
    directory: PathBuf,
    fetch: Fetch,
}

impl<K, Fetch, FetchReturn, FetchOutput> FileCache<K, Fetch, FetchReturn, FetchOutput>
where
    K: Key + Copy,
    Fetch: Fn(K) -> FetchReturn,
    FetchReturn: Future<Output = anyhow::Result<FetchOutput>>,
    FetchOutput: Stream<Item = anyhow::Result<Bytes>> + std::marker::Unpin,
{
    pub async fn new<P: Into<PathBuf>>(directory: P, fetch: Fetch) -> anyhow::Result<Self> {
        let directory: PathBuf = directory.into();
        if !directory.exists() {
            create_dir_all(&directory)
                .await
                .with_context(|| format!("creating cache directory {}", directory.display()))?;
        }
        Ok(Self {
            directory,
            fetch,
            key: PhantomData,
        })
    }

    pub async fn get(&self, key: &K) -> anyhow::Result<String> {
        let path = self.path_for_key(key);
        if !path.exists() {
            self.populate(key, &path).await?;
        }
        let input = tokio::fs::read(&path)
            .await
            .context(format!("read from {}", path.display()))?;
        Ok(String::from_utf8(input)?)
    }

    pub async fn populate(&self, key: &K, path: &PathBuf) -> anyhow::Result<()> {
        let mut source = (self.fetch)(*key).await?;
        let mut sink = File::create(path)
            .await
            .with_context(|| format!("creating file {}", path.display()))?;

        while let Some(bytes) = source.next().await {
            sink.write_all(bytes?.as_ref()).await?;
        }

        Ok(())
    }

    fn path_for_key(&self, key: &K) -> PathBuf {
        self.directory.join(key.serialize().as_ref())
    }
}
