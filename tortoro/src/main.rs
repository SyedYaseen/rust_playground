use anyhow::Context;
use clap::Parser;
use sha1::Digest;
use tortoro::torrent::{Args, Command, Torrent};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Decode { value } => {
            let v: serde_json::Value = serde_bencode::from_str(&value)?;
            println!("{:#?}", v);
        }
        Command::Info { torrent } => {
            let dot_torrent = std::fs::read(torrent.as_path())?;
            let t: Torrent = serde_bencode::from_bytes(&dot_torrent)?;
            println!("TrackerUrl: {}", &t.announce);

            let info_encoded = serde_bencode::to_bytes(&t.info).context("reencode info dict")?;
            let mut hasher = sha1::Sha1::new();
            hasher.update(&info_encoded);
            let info_hash = hasher.finalize();

            println!("Info Hash: {}", hex::encode(info_hash));
            println!("PieceLen: {}", &t.info.piece_length);
            for h in &t.info.pieces.0 {
                println!("{}", hex::encode(&h));
            }
        }
    }

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::decode_bencoded_value;

//     #[test]
//     fn decodes_a_string() {
//         assert_eq!(
//             decode_bencoded_value("5:hello").unwrap(),
//             (serde_json::json!("hello"), "")
//         );
//     }

//     #[test]
//     fn decodes_an_neg_integer() {
//         assert_eq!(
//             decode_bencoded_value("i-42e").unwrap(),
//             (serde_json::json!(-42), "")
//         );
//     }

//     #[test]
//     fn decodes_an_integer() {
//         assert_eq!(
//             decode_bencoded_value("i42e").unwrap(),
//             (serde_json::json!(42), "")
//         );
//     }

//     #[test]
//     fn decodes_a_list() {
//         assert_eq!(
//             decode_bencoded_value("l5:helloi42ee").unwrap(),
//             (serde_json::json!(["hello", 42]), "")
//         );
//     }

//     #[test]
//     fn decodes_a_dictionary() {
//         assert_eq!(
//             decode_bencoded_value("d3:foo3:bar5:helloi52ee").unwrap(),
//             (serde_json::json!({ "foo": "bar", "hello": 52 }), "")
//         );
//     }
// }
