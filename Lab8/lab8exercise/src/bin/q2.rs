use csv::{ReaderBuilder, Writer, Trim};
use std::io::Read;

struct Point {
    x: f64,
    y: f64,
    tag_color: String,
}

fn euclidean_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

fn load_points<R: Read>(rdr: R) -> Vec<Point> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(rdr);
    let mut tagged_points = Vec::new();

    for record in reader.records() {
        if let Ok(rec) = record {
            if rec.len() >= 2 {
                let x: f64 = rec[0].parse().unwrap();
                let y: f64 = rec[1].parse().unwrap();
                let distance = euclidean_distance(x, y, 0.0, 0.0);

                let tag_color = if distance <= 1.0 {
                    "#80FF8080".to_string()
                } else {
                    "#FF808080".to_string()
                };

                tagged_points.push(Point {
                    x,
                    y,
                    tag_color,
                });
            }
        }
    }

    tagged_points
}

fn main() {
    let data = "\
        1.0, 2.0\n\
        3.0, 4.0\n\
    "
    .as_bytes();

    let tagged_points = load_points(data);

    for point in &tagged_points {
        let formatted_output = format!("({:.1}, {:.1}) {}", point.x, point.y, point.tag_color);
        println!("{}", formatted_output);
    }
}

#[test]
    fn test_load_points() {
        let data = "\
            1.0, 2.0\n\
            3.0, 4.0\n\
            0.0, 0.0\n\
        "
        .as_bytes();

        let tagged_points = load_points(data);

        assert_eq!(tagged_points.len(), 3);
        assert_eq!(tagged_points[0].x, 1.0);
        assert_eq!(tagged_points[0].y, 2.0);
        assert_eq!(tagged_points[0].tag_color, "#FF808080");

        assert_eq!(tagged_points[1].x, 3.0);
        assert_eq!(tagged_points[1].y, 4.0);
        assert_eq!(tagged_points[1].tag_color, "#FF808080");

        assert_eq!(tagged_points[2].x, 0.0);
        assert_eq!(tagged_points[2].y, 0.0);
        assert_eq!(tagged_points[2].tag_color, "#80FF8080");
    }






