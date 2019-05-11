// Represents a 2D Point.
#[derive(Debug)]
pub struct Point2D {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Line2D {
    points: Vec<Point2D>,
}

#[derive(Debug)]
pub struct MultiLine2D {
    lines: Vec<Line2D>,
}

/// Convert a set of coordinates into a 2d point.
pub fn geo_shape_coordinate_to_point_2d(lon: f64, lat: f64) -> Point2D {
    // assertions.
    assert!(lon >= -7.0, "lon should be between -7 and +10");
    assert!(lon <= 10.0, "lon should be between -7 and +10");
    assert!(lat >= 40.0, "lat should be between 40 and 52");
    assert!(lat <= 52.0, "lat should be between 40 and 52");

    let x = (1000000.0 * lon).abs() as i32;
    let y = (1000000.0 * lat).abs() as i32;

    Point2D { x, y }
}

/// Convert a geoshape line to a 2D line.
pub fn geo_shape_to_line_2d(coords: &Vec<[f64; 3]>) -> Line2D {
    let points = coords
        .into_iter()
        .map(|c| geo_shape_coordinate_to_point_2d(c[0], c[1]))
        .collect();
    Line2D { points }
}

/// Convert a set of geoshape lines to a multi 2D line.
pub fn geo_shape_to_multi_line_2d(multi_coords: &Vec<Vec<[f64; 3]>>) -> MultiLine2D {
    let lines = multi_coords
        .into_iter()
        .map(|mc| geo_shape_to_line_2d(mc))
        .collect();
    MultiLine2D { lines }
}
