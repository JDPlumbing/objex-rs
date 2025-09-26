use objex::{Objex, Shape, MaterialLink, insert_objex, fetch_objex};
use supabasic::Supabase;
use uuid::Uuid;
use serde_json::json;
use std::env;
use dotenvy::dotenv;

fn supabase_client() -> Supabase {
    dotenv().ok(); // load .env if not already loaded
    let url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set");
    Supabase::new(&url, &key)
}

#[tokio::test]
async fn test_insert_and_fetch_objex() {
    let supa = supabase_client();

    let entity_id = Uuid::new_v4();
    let dummy_shape = Shape {
        geometry: json!({
            "type": "cube",
            "dimensions": { "x": 1.0, "y": 2.0, "z": 3.0 }
        }),
    };
    let dummy_material = MaterialLink {
        category_id: Uuid::parse_str("cccccccc-cccc-cccc-cccc-cccccccc0003").unwrap(),
        properties: json!({ "density": 7.85 }),
    };

    let obj = Objex {
        entity_id,
        name: droidid::generate(),
        shape: dummy_shape.clone(),
        material: dummy_material.clone(),
    };

    insert_objex(&supa, entity_id, &obj).await.expect("insert works");
    let fetched = fetch_objex(&supa, entity_id).await.expect("fetch works");

    assert_eq!(fetched.shape.geometry, dummy_shape.geometry);
    assert_eq!(fetched.material.category_id, dummy_material.category_id);
}
