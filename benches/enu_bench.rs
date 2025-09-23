use criterion::{criterion_group, criterion_main, Criterion, black_box};
use uvoxid::UvoxId;
use uvoxxyz::types::Cartesian;
use uvoxxyz::enu::{to_local_enu, from_local_enu};

fn bench_enu(c: &mut Criterion) {
    let anchor = UvoxId::earth(6_371_000_000, 45_000_000, 90_000_000); // 45N, 90E
    let offset = Cartesian { x: 10.0, y: 5.0, z: 2.0 };

    c.bench_function("to_local_enu", |b| {
        b.iter(|| {
            let global = UvoxId::earth(6_371_000_000 + 1_000, 45_000_000, 90_000_000);
            black_box(to_local_enu(black_box(&anchor), black_box(&global)));
        })
    });

    c.bench_function("from_local_enu", |b| {
        b.iter(|| {
            black_box(from_local_enu(black_box(&anchor), black_box(offset)));
        })
    });
}

criterion_group!(benches, bench_enu);
criterion_main!(benches);
