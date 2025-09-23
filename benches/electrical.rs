use criterion::{criterion_group, criterion_main, Criterion};
use objex::systems::electrical::{derive_electrical, ElectricalProps};
use objex::core::Object;
use geospec::Cylinder;
use matcat::MatCatId;

fn bench_electrical(c: &mut Criterion) {
    // Copper-like material (metal category)
    let copper_id = MatCatId::new(0x01, 0, 0);
    let copper_obj = Object::new(Cylinder { radius: 0.05, height: 1.0 }, copper_id);

    // Plastic-like material (plastic category)
    let plastic_id = MatCatId::new(0x04, 0, 0);
    let plastic_obj = Object::new(Cylinder { radius: 0.05, height: 1.0 }, plastic_id);

    c.bench_function("derive_electrical (100k copper cylinders)", |b| {
        b.iter(|| {
            for _ in 0..100_000 {
                let _elec: ElectricalProps = derive_electrical(&copper_obj);
            }
        })
    });

    c.bench_function("derive_electrical (100k plastic cylinders)", |b| {
        b.iter(|| {
            for _ in 0..100_000 {
                let _elec: ElectricalProps = derive_electrical(&plastic_obj);
            }
        })
    });
}

criterion_group!(benches, bench_electrical);
criterion_main!(benches);
