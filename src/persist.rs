use supabasic::Supabase;
use uuid::Uuid;
use serde_json::json;

use crate::{Objex, Shape, MaterialLink};
use crate::error::ObjexError;

/// Insert or update an Objex into entities + geospec + matcat_link
pub async fn insert_objex(
    supa: &Supabase,
    entity_id: Uuid,
    objex: &Objex,
) -> Result<(), ObjexError> {
    // Insert into entities
    supa.from("entities")
        .insert(json!([{
            "id": entity_id,
            "name": objex.name.clone(),
        }]))
        .execute()
        .await?;

    // Insert into geospec
    supa.from("geospec")
        .insert(json!([{
            "entity_id": entity_id,
            "shape": objex.shape,
        }]))
        .execute()
        .await?;

    // Insert into matcat_link
    supa.from("matcat_link")
        .insert(json!([{
            "entity_id": entity_id,
            "category_id": objex.material.category_id,
            "properties": objex.material.properties,
        }]))
        .execute()
        .await?;

    Ok(())
}

/// Fetch an Objex (joins entities + geospec + matcat_link)
/// Fetch an Objex (joins entities + geospec + matcat_link)
pub async fn fetch_objex(
    supa: &Supabase,
    entity_id: Uuid,
) -> Result<Objex, ObjexError> {
    // Fetch from entities
    let ent_val: serde_json::Value = supa
        .from("entities")
        .select("name")
        .eq("id", &entity_id.to_string())
        .execute()
        .await?;

    let ent_row = ent_val
        .as_array()
        .and_then(|arr| arr.first())
        .ok_or_else(|| ObjexError::Other(anyhow::anyhow!("no entity found")))?;

    // Fetch from geospec
    let shape_val: serde_json::Value = supa
        .from("geospec")
        .select("shape")
        .eq("entity_id", &entity_id.to_string())
        .execute()
        .await?;

    let shape_row = shape_val
        .as_array()
        .and_then(|arr| arr.first())
        .ok_or_else(|| ObjexError::Other(anyhow::anyhow!("no geospec row found")))?;

    // Fetch from matcat_link
    let matcat_val: serde_json::Value = supa
        .from("matcat_link")
        .select("category_id, properties")
        .eq("entity_id", &entity_id.to_string())
        .execute()
        .await?;

    let matcat_row = matcat_val
        .as_array()
        .and_then(|arr| arr.first())
        .ok_or_else(|| ObjexError::Other(anyhow::anyhow!("no matcat_link row found")))?;

    Ok(Objex {
        entity_id,
        name: ent_row["name"].as_str().unwrap_or("Unnamed Objex").to_string(),
        shape: serde_json::from_value(shape_row["shape"].clone())?,
        material: MaterialLink {
            category_id: Uuid::parse_str(
                matcat_row["category_id"]
                    .as_str()
                    .ok_or_else(|| ObjexError::Other(anyhow::anyhow!("missing category_id")))?,
            )?,
            properties: matcat_row["properties"].clone(),
        },
    })
}
