use math;

// Represents a 2D Point.
#[derive(Debug, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }
}

#[derive(Debug)]
pub struct Line {
    points: Vec<Point>,
}

#[derive(Debug)]
pub struct Network {
    // list of all lines.
    lines: Vec<Line>,

    // list of all points.
    points: Vec<Point>,
}

impl Network {
    pub fn get_line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    pub fn new() -> Network {
        Network {
            lines: vec![],
            points: vec![],
        }
    }
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
pub fn geo_shape_to_multi_line(multi_coords: &Vec<Vec<[f64; 3]>>) -> Vec<Line> {
    let lines = multi_coords
        .into_iter()
        .map(|mc| geo_shape_to_line(mc))
        .collect();

    lines
}

/// round
fn round(v: f64) -> f64 {
    math::round::half_down(v, 5)
}

/// round a vector
fn vector_round(v: (f64, f64)) -> (f64, f64) {
    (round(v.0), round(v.1))
}

/// Length of the vector.
fn vector_length(u: (f64, f64)) -> f64 {
    (u.0 * u.0 + u.1 * u.1).sqrt()
}

fn vector_dot_product(u: (f64, f64), v: (f64, f64)) -> f64 {
    ((u.0 * v.0) + (u.1 * v.1))
}

/// Angle of the vector.
fn vector_angle(u: (f64, f64), v: (f64, f64)) -> f64 {
    (vector_dot_product(u, v) / (vector_length(u) * vector_length(v))).acos()
}

/// Rotate the vector
fn vector_rotate(v: (f64, f64), teta: f64) -> (f64, f64) {
    (
        v.0 * teta.cos() - v.1 * teta.sin(),
        v.0 * teta.sin() + v.1 * teta.cos(),
    )
}

/// convert a point to a vector.
fn convert_point_to_vector(p: Point) -> (f64, f64) {
    (p.x as f64, p.y as f64)
}

/// merge (or not) the point with the segment.
fn can_merge_segment_with_point(from: Point, to: Point, point: Point, distance: i32) -> bool {
    true
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_convert() {
        assert_eq!(
            geo_shape_coordinate_to_point(-7.0, 52.0, 0.0),
            Point { x: 0, y: 0, z: 0 }
        );

        assert_eq!(
            geo_shape_coordinate_to_point(10.0, 40.0, 0.0),
            Point {
                x: 1300,
                y: 1300,
                z: 0
            }
        );

        assert_eq!(
            geo_shape_coordinate_to_point(10.0, 40.0, 100.0),
            Point {
                x: 1300,
                y: 1300,
                z: 1
            }
        );
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

    #[test]
    fn test_can_merge_segment_and_point() {
        assert!(can_merge_segment_with_point(
            Point::new(0, 0, 0),
            Point::new(10, 0, 0),
            Point::new(5, 0, 0),
            10
        ));

        assert!(can_merge_segment_with_point(
            Point::new(0, 0, 0),
            Point::new(0, 10, 0),
            Point::new(0, 5, 0),
            10
        ));

        assert!(can_merge_segment_with_point(
            Point::new(0, 0, 0),
            Point::new(10, 10, 0),
            Point::new(5, 5, 0),
            10
        ));
    }

    #[test]
    fn test_vector_computations() {
        assert_eq!(vector_length((0.0, 1.0)), 1.0);
        assert_eq!(vector_length((1.0, 0.0)), 1.0);
        assert_eq!(vector_length((-1.0, 0.0)), 1.0);
        assert_eq!(vector_length((0.0, -1.0)), 1.0);

        assert_eq!(round(vector_angle((0.0, 1.0), (1.0, 0.0))), round(PI / 2.0));
        assert_eq!(round(vector_angle((1.0, 1.0), (0.0, 1.0))), round(PI / 4.0));
        assert_eq!(round(vector_angle((0.0, 1.0), (0.0, 1.0))), round(0.0));

        assert_eq!(
            vector_round(vector_rotate((0.0, 1.0), -PI / 2.0)),
            (1.0, 0.0)
        );
        assert_eq!(
            vector_round(vector_rotate((0.0, 1.0), -PI / 4.0)),
            (0.70711, 0.70711)
        );

        assert_eq!(
            round(vector_angle((1.0, 0.0), (-1.0, 1.0))),
            round(PI * 3.0 / 4.0)
        );
    }
}
