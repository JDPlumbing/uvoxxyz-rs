use uvoxxyz::types::Cartesian;
use uvoxxyz::quat::Quat;
use std::f64::consts::PI;

#[test]
fn identity_rotation_does_nothing() {
    let q = Quat::identity();
    let v = Cartesian { x: 1.0, y: 0.0, z: 0.0 };
    let rotated = q.rotate(v);
    assert!((rotated.x - v.x).abs() < 1e-9);
    assert!((rotated.y - v.y).abs() < 1e-9);
    assert!((rotated.z - v.z).abs() < 1e-9);

}

#[test]
fn rotate_90_deg_about_z() {
    let axis = Cartesian { x: 0.0, y: 0.0, z: 1.0 };
    let q = Quat::from_axis_angle(axis, PI / 2.0);
    let v = Cartesian { x: 1.0, y: 0.0, z: 0.0 };
    let rotated = q.rotate(v);

    // Should be roughly (0,1,0)
    assert!((rotated.x).abs() < 1e-6);
    assert!((rotated.y - 1.0).abs() < 1e-6);
    assert!((rotated.z).abs() < 1e-6);
}
