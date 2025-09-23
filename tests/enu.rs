use uvoxid::UvoxId;
use uvoxxyz::types::Cartesian;
use uvoxxyz::enu::{to_local_enu, from_local_enu};

const EPS: f64 = 1e-3; // allow ~1 mm tolerance

#[test]
fn anchor_is_origin() {
    let anchor = UvoxId::earth(6_371_000_000, 0, 0); // Equator, lon=0
    let local = to_local_enu(&anchor, &anchor);
    assert!(local.x.abs() < EPS && local.y.abs() < EPS && local.z.abs() < EPS);
}

#[test]
fn east_offset_is_positive_x() {
    let anchor = UvoxId::earth(6_371_000_000, 0, 0);
    let east_point = UvoxId::earth(6_371_000_000, 0, 1_000); // small lon offset
    let local = to_local_enu(&anchor, &east_point);
    assert!(local.x > 0.0);
}

#[test]
fn north_offset_is_positive_y() {
    let anchor = UvoxId::earth(6_371_000_000, 0, 0);
    let north_point = UvoxId::earth(6_371_000_000, 1_000, 0); // small lat offset
    let local = to_local_enu(&anchor, &north_point);
    assert!(local.y > 0.0);
}

#[test]
fn up_offset_is_positive_z() {
    let anchor = UvoxId::earth(6_371_000_000, 0, 0);
    let higher = UvoxId::earth(6_371_000_000 + 1_000, 0, 0); // radial +1mm
    let local = to_local_enu(&anchor, &higher);
    assert!(local.z > 0.0);
}

#[test]
fn round_trip_local_global() {
    let anchor = UvoxId::earth(6_371_000_000, 45_000_000, 90_000_000); // 45N, 90E
    let offset = Cartesian { x: 10.0, y: 5.0, z: 2.0 };

    let global = from_local_enu(&anchor, offset);
    let back = to_local_enu(&anchor, &global);

    assert!((offset.x - back.x).abs() < EPS);
    assert!((offset.y - back.y).abs() < EPS);
    assert!((offset.z - back.z).abs() < EPS);
}

#[test]
fn enu_behaves_at_north_pole() {
    // Anchor at ~North Pole
    let anchor = UvoxId::earth(6_371_000_000, 90_000_000, 0); // lat = 90°, lon = 0
    let offset = Cartesian { x: 10.0, y: 0.0, z: 0.0 };

    // Go 10m "east" in local ENU
    let global = from_local_enu(&anchor, offset);
    let back = to_local_enu(&anchor, &global);

    // Round-trip should stay within tolerance
    assert!((offset.x - back.x).abs() < EPS, "East drift too large at pole");
    assert!((offset.y - back.y).abs() < EPS, "North drift too large at pole");
    assert!((offset.z - back.z).abs() < EPS, "Up drift too large at pole");
}

#[test]
fn enu_behaves_at_south_pole() {
    // Anchor at ~South Pole
    let anchor = UvoxId::earth(6_371_000_000, -90_000_000, 0); // lat = -90°, lon = 0
    let offset = Cartesian { x: 0.0, y: 10.0, z: 0.0 };

    // Go 10m "north" in local ENU
    let global = from_local_enu(&anchor, offset);
    let back = to_local_enu(&anchor, &global);

    // Round-trip should stay within tolerance
    assert!((offset.x - back.x).abs() < EPS, "East drift too large at pole");
    assert!((offset.y - back.y).abs() < EPS, "North drift too large at pole");
    assert!((offset.z - back.z).abs() < EPS, "Up drift too large at pole");
}
