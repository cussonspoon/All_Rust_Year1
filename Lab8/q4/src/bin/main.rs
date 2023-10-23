use csv::{ReaderBuilder, Writer, Trim};
use std::io::Read;
use std::fs::File;

fn main(){
    get_args()
}
#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
    color: String,
}

fn tag_point(pntlist: Vec<Point>) -> Vec<Point> {
    let unit_circle = 1.0;

    let mut points = Vec::new();

    for pnt in pntlist {
        let distance = (pnt.x.powi(2)+pnt.y.powi(2)).sqrt();
        if distance <= unit_circle {
            let point = Point {
                x: pnt.x,
                y: pnt.y,
                color: "#80FF8080".to_string(),
            };
            points.push(point);
        } else {
            let point = Point {
                x: pnt.x,
                y: pnt.y,
                color: "#FF808080".to_string(),
            };
            points.push(point);
        }

}
    points
}


#[test]
fn test_tag_points() {
    let input_points = vec![
        Point { x: 0.5, y: 0.5, color: String::new() },
        Point { x: 1.5, y: 1.5, color: String::new() },
    ];

    let tagged_points = tag_point(input_points);

    let expected_output = vec![
        Point { x: 0.5, y: 0.5, color: "#80FF8080".to_string() },
        Point { x: 1.5, y: 1.5, color: "#FF808080".to_string() },
    ];

    assert_eq!(tagged_points, expected_output);
}

fn load_pairs<R: Read>(rdr: R) -> Vec<Point> {
    let mut reader
        = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(rdr);
    let mut out_list = vec![];
        for record in reader.records() {
        if let Ok(rec) = record {
        let x: f64 = rec[0].parse().unwrap();
        let y: f64 = rec[1].parse().unwrap();
        out_list.push(Point { x: x, y: y, color: "#000000".to_string() });
        }
    }
    out_list
}

#[test]
fn test_load_pairs() {

    let csv_data = "1.0,2.0\n3.0,4.0\n";
    let input_file_path = csv_data.as_bytes();

    let pairs = load_pairs(input_file_path);

    let expected_output = vec![
        Point { x: 1.0, y: 2.0, color: "#000000".to_string() },
        Point { x: 3.0, y: 4.0, color: "#000000".to_string() },
    ];

    assert_eq!(pairs, expected_output);
}

fn save_points<W: std::io::Write>(writer: W, pt_list: Vec<Point>) {
    let mut wtr = Writer::from_writer(writer);
    for pt in pt_list {
        wtr.write_record(&[pt.x.to_string(), pt.y.to_string(), pt.color]).unwrap();
    }
    wtr.flush().unwrap();
}

#[test]
fn test_save_points() {
    let pt_list = vec![
        Point {
            x: 0.0,
            y: 0.0,
            color: String::new(),
        },
        Point {
            x: 0.5,
            y: 0.5,
            color: String::new(),
        },
        Point {
            x: 1.0,
            y: 1.0,
            color: String::new(),
        },
    ];
    let mut result = Vec::new();
    save_points(&mut result, pt_list);
    let result = String::from_utf8(result).unwrap();
    assert_eq!(result, "0,0,\n0.5,0.5,\n1,1,\n");
}
    
fn get_args() {
    let args:Vec<_> = std::env::args().collect();
    let input_name: String = args[1].parse().unwrap();
    let output_name: String = args[2].parse().unwrap();
    
    let point = load_pairs(File::open(input_name).unwrap());
    let tagged_points = tag_point(point);
    let result = save_points(File::create(output_name).unwrap(), tagged_points);
    
    result
}
