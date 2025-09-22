use objex::core::{Object, CompositeObject};
use objex::systems::*;
use geospec::Cylinder;
use matcat::MatCatId;

#[test]
fn compare_mechanical_props() {
    // Single material: steel
    let steel = Object::new(Cylinder { radius: 0.05, height: 1.0 }, MatCatId::new(1, 1, 0));
    let steel_mech = derive_mechanical(&steel);

    println!("\n=== Steel Cylinder ===");
    println!("Young’s Modulus: {:.2e} Pa", steel_mech.youngs_modulus);
    println!("Hardness: {}", steel_mech.hardness);
    println!("Fracture Toughness: {:.1} MPa·m^0.5", steel_mech.fracture_toughness);
    println!("Inertia: {:.2} kg·m²", steel_mech.inertia);

    // Composite: steel + PVC coating
    let pvc = Object::new(Cylinder { radius: 0.051, height: 1.0 }, MatCatId::new(3, 1, 0));
    let pipe = CompositeObject::new(vec![steel, pvc]);
    let pipe_mech = derive_mechanical_composite(&pipe);

    println!("\n=== Steel + PVC Pipe ===");
    println!("Composite Young’s Modulus: {:.2e} Pa", pipe_mech.youngs_modulus);
    println!("Composite Hardness: {}", pipe_mech.hardness);
    println!("Composite Fracture Toughness: {:.1} MPa·m^0.5", pipe_mech.fracture_toughness);
    println!("Composite Inertia: {:.2} kg·m²", pipe_mech.inertia);
}
