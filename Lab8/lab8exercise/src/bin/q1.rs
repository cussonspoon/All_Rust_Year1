struct Point {
    x: f64,
    y: f64,
    tag_color: String,
}

fn euclidean_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

fn tag_points(pt_list: Vec<Point>) -> Vec<Point> {
    let mut tagged_points = Vec::new();

    for point in pt_list {
        let distance = euclidean_distance(point.x, point.y, 0.0, 0.0);

        let tag_color = if distance <= 1.0 {
            "#80FF8080".to_string()
        } else {
            "#FF808080".to_string()
        };

        tagged_points.push(Point {
            x: point.x,
            y: point.y,
            tag_color,
        });
    }

    tagged_points
}

fn main() {
    let points = vec![
        Point { x: 0.5, y: 0.5, tag_color: String::new() },
        Point { x: 1.2, y: 0.0, tag_color: String::new() },
        Point { x: -0.7, y: -0.7, tag_color: String::new() },
        Point { x: -1.1, y: 4.0, tag_color: String::new()}
    ];

    let tagged_points = tag_points(points);

    for point in tagged_points {
        println!("({}, {}) {}", point.x, point.y, point.tag_color);
    }
}

#[test]
fn test_tag_points() {
    let points = vec![
        Point { x: 0.5, y: 0.5, tag_color: String::new() },
        Point { x: 1.2, y: 0.0, tag_color: String::new() },
        Point { x: -0.7, y: -0.7, tag_color: String::new() },
        Point { x: -1.1, y: 4.0, tag_color: String::new() },
    ];

    let tagged_points = tag_points(points);

    assert_eq!(tagged_points.len(), 4);
    assert_eq!(tagged_points[0].tag_color, "#80FF8080");
    assert_eq!(tagged_points[1].tag_color, "#FF808080");
    assert_eq!(tagged_points[2].tag_color, "#80FF8080");
    assert_eq!(tagged_points[3].tag_color, "#FF808080");
}
