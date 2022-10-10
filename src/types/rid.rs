use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub struct RecordID {
    bucket_id: i32,
    record_position: i64,
}

impl RecordID {
    pub fn new(bucket_id: i32, record_position: i64) -> Self {
        Self {
            bucket_id,
            record_position,
        }
    }
}

impl Serialize for RecordID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("#{}:{}", self.bucket_id, self.record_position))
    }
}

impl<'de> Deserialize<'de> for RecordID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(RecordIDVisitor)
    }
}

struct RecordIDVisitor;

impl<'v> Visitor<'v> for RecordIDVisitor {
    type Value = RecordID;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "A RecordID with format #<bucket-identifier>:<record-position>"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v
            .split_terminator(&['#', ':'])
            .collect::<Vec<&str>>()
            .as_slice()
        {
            ["", bucket_id, record_position] => {
                let bucket = bucket_id.parse().map_err(E::custom)?;
                let position = record_position.parse().map_err(E::custom)?;
                Ok(RecordID::new(bucket, position))
            }

            _ => Err(E::custom(
                "Invalid RecordID, expected format #<bucket-identifier>:<record-position>",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;
    use serde_test::{self, Configure, Readable, Token};

    use crate::RecordID;

    #[test]
    fn should_deserialize_valid_rid() {
        let rid_str = "#1:10";
        let rid = RecordID::new(1, 10);
        serde_test::assert_tokens(&rid.readable(), &[Token::Str(rid_str)]);
    }
    #[test]
    fn should_deserialize_null_rid() {
        let rid_str = "#-1:-1";
        let rid = RecordID::new(-1, -1);
        serde_test::assert_tokens(&rid.readable(), &[Token::Str(rid_str)]);
    }
    #[test]
    fn should_fail_to_deserialize_rid_with_missing_pound() {
        assert_token_error(
            "1:10",
            "Invalid RecordID, expected format #<bucket-identifier>:<record-position>",
        );
    }
    #[test]
    fn should_fail_to_deserialize_rid_with_invalid_rid() {
        assert_token_error("#hello:32", "invalid digit found in string");
        assert_token_error("#32:hello", "invalid digit found in string");
        assert_token_error(
            "#1:",
            "Invalid RecordID, expected format #<bucket-identifier>:<record-position>",
        );
        assert_token_error(
            "#1",
            "Invalid RecordID, expected format #<bucket-identifier>:<record-position>",
        );
    }

    fn assert_token_error(rid: &'static str, error: &str) {
        serde_test::assert_de_tokens_error::<Readable<RecordID>>(&[Token::Str(rid)], error);
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct Record {
        id: RecordID,
    }

    #[test]
    fn should_deserialize_json_with_rid() {
        let json = json!({ "id" : "#10:10" });

        let deserialized = serde_json::from_value(json).unwrap();

        assert_eq!(
            Record {
                id: RecordID::new(10, 10)
            },
            deserialized
        );
    }
    #[test]
    fn should_fail_to_deserialize_json_with_with_invalid_rid() {
        let json = json!({ "id" : 32 });

        let deserialized = serde_json::from_value::<Record>(json).unwrap_err();

        assert_eq!(
            "invalid type: integer `32`, expected A RecordID with format #<bucket-identifier>:<record-position>",
            deserialized.to_string()
        );

        let json = json!({ "id" : "#32" });

        let deserialized = serde_json::from_value::<Record>(json).unwrap_err();

        assert_eq!(
            "Invalid RecordID, expected format #<bucket-identifier>:<record-position>",
            deserialized.to_string()
        );
    }
}
