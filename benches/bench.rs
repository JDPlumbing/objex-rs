use criterion::{criterion_group, criterion_main, Criterion};
use objex::core::Object;
use objex::systems::{derive_mass, derive_strength, derive_thermal, derive_degradation};
use geospec::Cylinder;
use matcat::MatCatId;

fn param_sweep_with_output(c: &mut Criterion) {
    let steel = MatCatId::new(1, 1, 0);

    let radii = [0.01, 0.05, 0.1, 0.5, 1.0];
    let heights = [0.1, 0.5, 1.0, 5.0, 10.0];

    c.bench_function("param_sweep cylinders w/ output", |b| {
        b.iter(|| {
            let mut masses = Vec::new();
            let mut failures = Vec::new();
            let mut resistances = Vec::new();
            let mut lifespans = Vec::new();

            for &r in &radii {
                for &h in &heights {
                    let cyl = Cylinder { radius: r, height: h };
                    let obj = Object::new(cyl, steel);

                    let m = derive_mass(&obj);
                    let s = derive_strength(&obj);
                    let t = derive_thermal(&obj, 0.01);
                    let d = derive_degradation(&obj);

                    masses.push(m.mass);
                    failures.push(s.failure_load);
                    resistances.push(t.thermal_resistance);
                    lifespans.push(d.estimated_lifespan_years);
                }
            }

            // print ranges (cargo bench -- --nocapture to see)
            let min_mass = masses.iter().cloned().fold(f64::INFINITY, f64::min);
            let max_mass = masses.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

            let min_fail = failures.iter().cloned().fold(f64::INFINITY, f64::min);
            let max_fail = failures.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

            let min_res = resistances.iter().cloned().fold(f64::INFINITY, f64::min);
            let max_res = resistances.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

            let min_life = lifespans.iter().cloned().fold(f64::INFINITY, f64::min);
            let max_life = lifespans.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

            println!("\n=== Sweep Results ===");
            println!("Mass range: {:.2} – {:.2} kg", min_mass, max_mass);
            println!("Failure Load range: {:.2} – {:.2} N", min_fail, max_fail);
            println!("Thermal Resistance range: {:.6} – {:.6} K/W", min_res, max_res);
            println!("Lifespan range: {:.1} – {:.1} years", min_life, max_life);
        })
    });
}

criterion_group!(benches, param_sweep_with_output);
criterion_main!(benches);
