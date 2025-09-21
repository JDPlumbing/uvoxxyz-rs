use uvoxid::UvoxId;
use uvoxxyz::types::{CoordSystem, Cartesian};
use uvoxxyz::convert::UvoxIdExt;

#[test]
fn round_trip_math_coords() {
    let original = UvoxId::earth(6_371_000_000, 45_000_000, 90_000_000); // ~Earth surface, 45°N, 90°E

    let cart = original.to_cartesian(CoordSystem::Math);
    let back = UvoxId::from_cartesian(cart, CoordSystem::Math, original.frame_id);

    // r_um within 1 µm
    assert!((original.r_um as i128 - back.r_um as i128).abs() <= 1);
    // lat/lon codes within 1 tick
    assert!((original.lat_code - back.lat_code).abs() <= 1);
    assert!((original.lon_code - back.lon_code).abs() <= 1);
}

#[test]
fn round_trip_graphics_coords() {
    let original = UvoxId::earth(6_371_000_000, -30_000_000, -45_000_000); // ~Earth surface, 30°S, 45°W

    let cart = original.to_cartesian(CoordSystem::Graphics);
    let back = UvoxId::from_cartesian(cart, CoordSystem::Graphics, original.frame_id);

    assert!((original.r_um as i128 - back.r_um as i128).abs() <= 1);
    assert!((original.lat_code - back.lat_code).abs() <= 1);
    assert!((original.lon_code - back.lon_code).abs() <= 1);
}

#[test]
fn zero_point_is_origin() {
    let id = UvoxId::earth(0, 0, 0);
    let cart = id.to_cartesian(CoordSystem::Math);

    assert!((cart.x.abs() < 1e-9) && (cart.y.abs() < 1e-9) && (cart.z.abs() < 1e-9));
}

#[test]
fn symmetry_across_equator() {
    let north = UvoxId::earth(1_000_000, 45_000_000, 0);
    let south = UvoxId::earth(1_000_000, -45_000_000, 0);

    let n_cart = north.to_cartesian(CoordSystem::Math);
    let s_cart = south.to_cartesian(CoordSystem::Math);

    // x same, y same, z opposite
    assert!((n_cart.x - s_cart.x).abs() < 1e-6);
    assert!((n_cart.y - s_cart.y).abs() < 1e-6);
    assert!((n_cart.z + s_cart.z).abs() < 1e-6);
}
