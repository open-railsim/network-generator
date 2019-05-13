// Represents a 2D Point.
#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
pub struct Line {
    points: Vec<Point>,
}

#[derive(Debug)]
pub struct MultiLine {
    lines: Vec<Line>,
}

/// Convert a set of coordinates into a point.
///
/// ```
/// 52 Y(lat)
///    |
///    |
///    |
///    |
///    |
/// 40 |------------X(lon)
///    -7           10
///
/// lat(40) -7 to 10 => 1445.86KM
/// lat(52) -7 to 10 => 1161.14KM
/// lon(*)  40 to 52 => 1334.34KM
///
/// Precision : 0.0001  is ~10m.
/// Precision : 0.00001 is ~1m.
/// ```
pub fn geo_shape_coordinate_to_point(lon: f64, lat: f64, elevation: f64) -> Point {
    // assertions.
    assert!(lon >= -7.0, "lon should be between -7 and +10");
    assert!(lon <= 10.0, "lon should be between -7 and +10");
    assert!(lat >= 40.0, "lat should be between 40 and 52");
    assert!(lat <= 52.0, "lat should be between 40 and 52");

    let factor = 100000.0;

    // adjust precision into meter.
    let a_min_lat = adjust_factor(52.0, factor);
    let a_max_lat = adjust_factor(40.0, factor);
    let a_min_lon = adjust_factor(-7.0, factor);
    let a_max_lon = adjust_factor(10.0, factor);

    let a_lat = adjust_factor(lat, factor);
    let a_lon = adjust_factor(lon, factor);
    let z = (elevation / 100.0).abs() as i32;

    println!("{:?}", a_lat);
    println!("{:?}", a_min_lat);
    println!("{:?}", a_max_lat);

    let x = normalize(a_min_lon, a_max_lon, 1300, a_lon);
    let y = normalize(a_min_lat, a_max_lat, 1300, a_lat);

    Point { x, y, z }
}

/// Adjust the value according a given factor.
fn adjust_factor(val: f64, factor: f64) -> i32 {
    (val * factor) as i32
}

/// Normalize a value.
///
/// ```
/// from |--------------| to
///           | v
///     0|--------------| size
///      |----| returned value.
/// ```
pub fn normalize(from: i32, to: i32, size: i32, pos: i32) -> i32 {
    // distance between from and to.
    let delta: f64;

    // distance between pos and from.
    let delta_pos: f64;

    // computed result.
    let result: f64;

    let mut x = from;
    let mut y = to;
    let mut p = pos;

    // guard.
    assert!(x != y);

    if to < from {
        x = to;
        y = from;
        p = to - pos + from;
    }

    delta = y as f64 - x as f64;
    delta_pos = p as f64 - x as f64;

    result = (delta_pos * size as f64) / delta;

    result as i32
}

/// Convert a geoshape line to a line.
pub fn geo_shape_to_line(coords: &Vec<[f64; 3]>) -> Line {
    let points = coords
        .into_iter()
        .map(|c| geo_shape_coordinate_to_point(c[0], c[1], c[2]))
        .collect();
    Line { points }
}

/// Convert a set of geoshape lines to a multi line.
pub fn geo_shape_to_multi_line(multi_coords: &Vec<Vec<[f64; 3]>>) -> MultiLine {
    let lines = multi_coords
        .into_iter()
        .map(|mc| geo_shape_to_line(mc))
        .collect();
    MultiLine { lines }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_convert() {
        let expected = Point { x: 0, y: 0, z: 0 };
        let point = geo_shape_coordinate_to_point(-7.0, 52.0, 0.0);

        assert_eq!(expected.x, point.x);
        assert_eq!(expected.y, point.y);
        assert_eq!(expected.z, point.z);
    }

    #[test]
    fn test_normalize() {

        assert_eq!(normalize(0, 10, 100, 0), 0);
        assert_eq!(normalize(0, 10, 100, 5), 50);
        assert_eq!(normalize(0, 10, 100, 10), 100);

        assert_eq!(normalize(30, 10, 100, 30), 0);
        assert_eq!(normalize(30, 10, 100, 20), 50);
        assert_eq!(normalize(30, 10, 100, 10), 100);

        assert_eq!(normalize(-10, 10, 100, -10), 0);
        assert_eq!(normalize(-10, 10, 100, 0), 50);
        assert_eq!(normalize(-10, 10, 100, 10), 100);

        assert_eq!(normalize(-7, 10, 1500, -7), 0);
        assert_eq!(normalize(-7, 10, 1500, 10), 1500);

        assert_eq!(normalize(52, 40, 1500, 52), 0);
        assert_eq!(normalize(52, 40, 1500, 40), 1500);

        assert_eq!(normalize(40, 52, 1500, 52), 1500);
        assert_eq!(normalize(40, 52, 1500, 40), 0);
    }
}
