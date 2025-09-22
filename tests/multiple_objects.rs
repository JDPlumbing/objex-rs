use objex::core::Object;
use objex::systems::{derive_mass, derive_strength, derive_thermal, derive_degradation};
use geospec::{Cylinder, BoxShape, Sphere};
use matcat::MatCatId;

#[test]
fn compare_variety_of_objects() {
    enum ShapeCase {
        Cyl(&'static str, Cylinder, MatCatId),
        Box(&'static str, BoxShape, MatCatId),
        Sph(&'static str, Sphere, MatCatId),
    }

    let cases = vec![
        ShapeCase::Cyl("Steel Cylinder", Cylinder { radius: 0.05, height: 1.0 }, MatCatId::new(1, 1, 0)),
        ShapeCase::Cyl("Aluminum Rod", Cylinder { radius: 0.02, height: 2.0 }, MatCatId::new(1, 2, 0)),
        ShapeCase::Cyl("Copper Pipe", Cylinder { radius: 0.03, height: 1.5 }, MatCatId::new(1, 3, 0)),

        ShapeCase::Box("Concrete Slab", BoxShape { length: 2.0, width: 1.0, height: 0.1 }, MatCatId::new(2, 1, 0)),

        ShapeCase::Sph("Plastic Sphere", Sphere { radius: 0.1 }, MatCatId::new(3, 1, 0)),
    ];

    for case in cases {
        match case {
            ShapeCase::Cyl(label, shape, mat) => run_case(label, Object::new(shape, mat)),
            ShapeCase::Box(label, shape, mat) => run_case(label, Object::new(shape, mat)),
            ShapeCase::Sph(label, shape, mat) => run_case(label, Object::new(shape, mat)),
        }
    }
}

fn run_case<T: geospec::Dimensions + geospec::Volume + geospec::SurfaceArea>(
    label: &str,
    obj: Object<T>,
) {
    let mass_props = derive_mass(&obj);
    let strength_props = derive_strength(&obj);
    let thermal_props = derive_thermal(&obj, 0.01);
    let degradation_props = derive_degradation(&obj);

    println!("\n=== {} ===", label);
    println!("Mass: {:.2} kg (ρ = {:.2} kg/m³)", mass_props.mass, mass_props.density);
    println!("Failure Load: {:.2} N", strength_props.failure_load);
    println!("Thermal Conductivity: {:.2} W/m·K", thermal_props.conductivity);
    println!("Estimated Lifespan: ~{:.1} years, ~{:.0} cycles",
             degradation_props.estimated_lifespan_years,
             degradation_props.estimated_lifespan_cycles);
}
