use objex::core::Object;
use objex::systems::{derive_mass, derive_strength, derive_thermal, derive_degradation};
use geospec::Cylinder;
use matcat::MatCatId;

#[test]
fn steel_cylinder_props() {
    // Example: Steel cylinder
    let steel = MatCatId::new(1, 1, 0); // 1=Metal, 1=Steel
    let cyl = Cylinder { radius: 0.05, height: 1.0 }; // 5 cm radius, 1 m tall

    let obj = Object::new(cyl, steel);

    let mass_props = derive_mass(&obj);
    let strength_props = derive_strength(&obj);
    let thermal_props = derive_thermal(&obj, 0.01); // 1 cm thickness
    let degradation_props = derive_degradation(&obj);

    // ==== Pretty report ====
    println!("\n=== Mass Properties ===");
    println!("Mass: {:.2} kg", mass_props.mass);
    println!("Density: {:.2} kg/m³", mass_props.density);
    println!("Volume: {:.5} m³", mass_props.volume);
    println!("Surface Area: {:.5} m²", mass_props.surface_area);
    println!("Surface/Volume: {:.2}", mass_props.surface_to_volume);

    println!("\n=== Strength Properties ===");
    println!("Tensile Strength: {:.2} MPa", strength_props.tensile_strength);
    println!("Compressive Strength: {:.2} MPa", strength_props.compressive_strength);
    println!("Failure Load (approx): {:.2} N", strength_props.failure_load);

    println!("\n=== Thermal Properties ===");
    println!("Conductivity: {:.2} W/m·K", thermal_props.conductivity);
    println!("Expansion Coeff: {:.2e} 1/K", thermal_props.expansion_coeff);
    println!("Melting Point: {:.1} °C", thermal_props.melting_point);
    println!("Thermal Resistance: {:.6} K/W", thermal_props.thermal_resistance);

    println!("\n=== Degradation Properties ===");
    println!("Fatigue Resistance: {:.3}", degradation_props.fatigue_resistance);
    println!("Corrosion Resistance: {:.3}", degradation_props.corrosion_resistance);
    println!("Estimated Lifespan: ~{:.0} cycles", degradation_props.estimated_lifespan_cycles);
    println!("Estimated Lifespan: ~{:.1} years", degradation_props.estimated_lifespan_years);

    // sanity checks
    assert!(mass_props.mass > 0.0);
    assert!(strength_props.tensile_strength > 0.0);
    assert!(thermal_props.melting_point > 0.0);
}
