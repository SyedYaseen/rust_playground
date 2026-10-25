use anyhow::{Result, anyhow, bail};
fn decode_bencoded_value(encoded_value: &str) -> Result<(serde_json::Value, &str)> {
    match encoded_value.as_bytes().first() {
        Some(b'i') => {
            // integer
            let end = encoded_value
                .find('e')
                .ok_or_else(|| anyhow!("failed to find 'e'"))?;
            let n = encoded_value[1..end].parse::<i64>()?;
            Ok((n.into(), &encoded_value[end + 1..]))
        }
        Some(b'0'..=b'9') => {
            // string
            let (len_str, rest) = encoded_value
                .split_once(":")
                .ok_or_else(|| anyhow!("Missing ':'"))?;

            let len: usize = len_str.parse()?;
            let value = rest
                .get(..len)
                .ok_or_else(|| anyhow!("string is shorter than its declared length"))?;
            Ok((value.into(), &rest[len..]))
        }
        Some(b'l') => {
            let mut res = Vec::new();
            let mut remainder = &encoded_value[1..];
            loop {
                if let Some(rest) = remainder.strip_prefix('e') {
                    return Ok((res.into(), rest));
                }
                if remainder.is_empty() {
                    bail!("list is missing its closing 'e'");
                }
                let (arr_val, rest) = decode_bencoded_value(remainder)?;
                res.push(arr_val);
                remainder = rest;
            }
        }
        Some(b'd') => {
            let mut res: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
            let mut remainder = &encoded_value[1..];

            let mut prev_key: Option<String> = None;

            loop {
                if let Some(rest) = remainder.strip_prefix('e') {
                    return Ok((res.into(), rest));
                }
                if remainder.is_empty() {
                    bail!("Dict is missing its closing 'e'");
                }

                let (dict_val, rest) = decode_bencoded_value(remainder)?;

                if let Some(prev) = prev_key {
                    res.entry(prev).or_insert(dict_val);
                    prev_key = None;
                } else {
                    let k = dict_val
                        .as_str()
                        .ok_or_else(|| anyhow!("Dict key must be string"))?
                        .to_owned();
                    prev_key = Some(k);
                }

                remainder = rest;
            }
        }

        _ => bail!("Invalid string input"),
    }
}

#[cfg(test)]
mod tests {
    use super::decode_bencoded_value;

    #[test]
    fn decodes_a_string() {
        assert_eq!(
            decode_bencoded_value("5:hello").unwrap(),
            (serde_json::json!("hello"), "")
        );
    }

    #[test]
    fn decodes_an_neg_integer() {
        assert_eq!(
            decode_bencoded_value("i-42e").unwrap(),
            (serde_json::json!(-42), "")
        );
    }

    #[test]
    fn decodes_an_integer() {
        assert_eq!(
            decode_bencoded_value("i42e").unwrap(),
            (serde_json::json!(42), "")
        );
    }

    #[test]
    fn decodes_a_list() {
        assert_eq!(
            decode_bencoded_value("l5:helloi42ee").unwrap(),
            (serde_json::json!(["hello", 42]), "")
        );
    }

    #[test]
    fn decodes_a_dictionary() {
        assert_eq!(
            decode_bencoded_value("d3:foo3:bar5:helloi52ee").unwrap(),
            (serde_json::json!({ "foo": "bar", "hello": 52 }), "")
        );
    }
}
