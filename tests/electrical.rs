use objex::core::Object;
use objex::systems::electrical::{derive_electrical, ElectricalProps};
use geospec::Cylinder;
use matcat::MatCatId;

#[test]
fn compare_electrical_props() {
    // Copper category (metal)
    let copper_id = MatCatId::new(0x01, 0, 0); // category=Metal
    let copper_obj = Object::new(
        Cylinder { radius: 0.05, height: 1.0 },
        copper_id,
    );
    let copper_elec = derive_electrical(&copper_obj);

    println!("=== Copper Cylinder ===");
    println!("{:?}", copper_elec);

    // Plastic category
    let plastic_id = MatCatId::new(0x04, 0, 0); // category=Plastic
    let plastic_obj = Object::new(
        Cylinder { radius: 0.05, height: 1.0 },
        plastic_id,
    );
    let plastic_elec = derive_electrical(&plastic_obj);

    println!("=== Plastic Cylinder ===");
    println!("{:?}", plastic_elec);

    // Sanity check: copper should conduct much better than plastic
    assert!(copper_elec.conductivity > plastic_elec.conductivity);
    assert!(copper_elec.resistance < plastic_elec.resistance);
}
