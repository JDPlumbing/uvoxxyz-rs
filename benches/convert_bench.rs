use criterion::{criterion_group, criterion_main, Criterion, black_box};
use uvoxid::UvoxId;
use uvoxxyz::types::CoordSystem;
use uvoxxyz::convert::UvoxIdExt;

fn bench_convert(c: &mut Criterion) {
    let id = UvoxId::earth(6_371_000_000, 45_000_000, 90_000_000);

    c.bench_function("uvoxid → cartesian (math)", |b| {
        b.iter(|| {
            let cart = id.to_cartesian(black_box(CoordSystem::Math));
            black_box(cart);
        })
    });

    c.bench_function("uvoxid → cartesian (graphics)", |b| {
        b.iter(|| {
            let cart = id.to_cartesian(black_box(CoordSystem::Graphics));
            black_box(cart);
        })
    });

    let cart_math = id.to_cartesian(CoordSystem::Math);
    let cart_graphics = id.to_cartesian(CoordSystem::Graphics);

    c.bench_function("cartesian → uvoxid (math)", |b| {
        b.iter(|| {
            let back = UvoxId::from_cartesian(black_box(cart_math), CoordSystem::Math, id.frame_id);
            black_box(back);
        })
    });

    c.bench_function("cartesian → uvoxid (graphics)", |b| {
        b.iter(|| {
            let back = UvoxId::from_cartesian(
                black_box(cart_graphics),
                CoordSystem::Graphics,
                id.frame_id,
            );
            black_box(back);
        })
    });
}

criterion_group!(benches, bench_convert);
criterion_main!(benches);
