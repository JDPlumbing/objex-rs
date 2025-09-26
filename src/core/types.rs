use serde::{Serialize, Deserialize};
use uuid::Uuid;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objex {
    pub entity_id: Uuid,
    pub name: String,             // now required
    pub shape: Shape,
    pub material: MaterialLink,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shape {
    pub geometry: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialLink {
    pub category_id: Uuid,
    pub properties: Value,
}
