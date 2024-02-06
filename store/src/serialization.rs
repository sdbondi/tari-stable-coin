// Copyright 2024 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use std::{any::type_name, str::FromStr};

use crate::error::StoreError;
use serde::Serialize;

pub fn serialize_json<T: Serialize + ?Sized>(t: &T) -> Result<String, StoreError> {
    serde_json::to_string_pretty(t).map_err(|e| StoreError::Encode {
        operation: "serialize_json",
        item: type_name::<T>(),
        details: e.to_string(),
    })
}

pub fn deserialize_json<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, StoreError> {
    serde_json::from_str(s).map_err(|e| StoreError::Decode {
        operation: "deserialize_json",
        item: type_name::<T>(),
        details: e.to_string(),
    })
}

pub fn serialize_hex<T: AsRef<[u8]>>(bytes: T) -> String {
    hex::encode(bytes.as_ref())
}

pub fn deserialize_hex(s: &str) -> Result<Vec<u8>, StoreError> {
    hex::decode(s).map_err(|e| StoreError::Decode {
        operation: "deserialize_hex",
        item: "Vec<u8>",
        details: e.to_string(),
    })
}

pub fn deserialize_hex_try_from<T>(s: &str) -> Result<T, StoreError>
where
    T: TryFrom<Vec<u8>>,
    T::Error: std::fmt::Display,
{
    let bytes = deserialize_hex(s)?;
    T::try_from(bytes).map_err(|e| StoreError::Decode {
        operation: "deserialize_hex_try_from",
        item: type_name::<T>(),
        details: e.to_string(),
    })
}

pub fn parse_from_string<T>(s: &str) -> Result<T, StoreError>
where
    T: FromStr,
{
    s.parse().map_err(|_| StoreError::Decode {
        operation: "parse_from_string",
        item: type_name::<T>(),
        details: format!("Cannot parse string '{s}'"),
    })
}
