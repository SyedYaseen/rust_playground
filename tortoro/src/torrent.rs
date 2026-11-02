use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Decode { value: String },
    Info { torrent: PathBuf },
}

#[derive(Debug, Clone, Deserialize)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

#[derive(Debug, Clone)]
pub struct Hashes(pub Vec<[u8; 20]>);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Info {
    pub name: String,

    #[serde(rename = "piece length")]
    pub piece_length: u64,
    pub pieces: Hashes,

    #[serde(flatten)]
    pub keys: Keys,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Keys {
    SingleFile { length: usize },
    MultiFile { files: Vec<TorrentFile> },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TorrentFile {
    pub length: u64,
    pub path: Vec<String>,
}

struct HashesVisitor;

impl<'de> Visitor<'de> for HashesVisitor {
    type Value = Hashes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a byte str of size 20")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> std::prelude::v1::Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() % 20 != 0 {
            return Err(E::custom(format!(
                "Invalid peice string. Len is {}",
                v.len()
            )));
        }
        let chunks = v.as_chunks().0.to_vec();
        Ok(Hashes(chunks))
    }
}

impl<'de> Deserialize<'de> for Hashes {
    fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(HashesVisitor)
    }
}

impl Serialize for Hashes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let concat_slice = self.0.concat();
        serializer.serialize_bytes(&concat_slice)
    }
}
