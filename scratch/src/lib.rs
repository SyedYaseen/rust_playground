use std::fmt;

use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};

/// A visitor that deserializes a long string - a string containing at least
/// some minimum number of bytes.
struct LongString {
    min: usize,
}

impl<'de> Visitor<'de> for LongString {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string containing at least {} bytes", self.min)
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if s.len() >= self.min {
            Ok(s.to_owned())
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        }
    }
}

/// Example 1: The validation belongs to the type itself.
///
/// Anywhere a `LongName` is deserialized, its inner value must contain at
/// least five bytes.
#[derive(Debug, PartialEq)]
struct LongName(String);

impl<'de> Deserialize<'de> for LongName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_str(LongString { min: 5 })
            .map(Self)
    }
}

// IN the simple exmaple if reusability isnt an issue, we wouldn't need a visitor impl or an additional struct. It could have been done like this.
// impl<'de> Deserialize<'de> for LongName {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let value = String::deserialize(deserializer)?;

//         if value.len() < 5 {
//             return Err(serde::de::Error::invalid_length(
//                 value.len(),
//                 &"a string containing at least 5 bytes",
//             ));
//         }

//         Ok(LongName(value))
//     }
// }

/// Example 2: The validation is attached only to this particular field.
///
/// `String` remains an ordinary string elsewhere in the program.
#[derive(Debug, Deserialize, PartialEq)]
struct FieldLevelExample {
    #[serde(deserialize_with = "deserialize_long_string")]
    name: String,
    age: u8,
}

fn deserialize_long_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(LongString { min: 5 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn struct_level_deserialization() {
        let name: LongName = serde_json::from_str(r#""Alice""#).unwrap();
        assert_eq!(name, LongName("Alice".to_owned()));

        let too_short = serde_json::from_str::<LongName>(r#""Ada""#);
        assert!(too_short.is_err());
    }

    #[test]
    fn field_level_deserialization() {
        let user: FieldLevelExample =
            serde_json::from_str(r#"{"name": "Alice", "age": 30}"#).unwrap();
        assert_eq!(user.name, "Alice");
        assert_eq!(user.age, 30);

        let too_short = serde_json::from_str::<FieldLevelExample>(r#"{"name": "Ada", "age": 30}"#);
        assert!(too_short.is_err());
    }
}
