use serde::de::IntoDeserializer;
use toml::Table;

use crate::prelude::*;

pub fn transcode_toml_to_message_pack(table: Table) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut serializer = rmp_serde::Serializer::new(&mut buffer).with_struct_map();
    serde_transcode::transcode(table.into_deserializer(), &mut serializer)
        .context("failed to transcode TOML table to MessagePack")?;
    Ok(buffer)
}
