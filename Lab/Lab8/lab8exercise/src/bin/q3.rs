use csv::{Writer, WriterBuilder};
use std::error::Error;
use std::fs::File;

#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
    tag_color: String,
}

fn format_point(point: &Point) -> String {
    format!("({:.1}, {:.1}) {}", point.x, point.y, point.tag_color)
}

fn save_points<W: std::io::Write>(mut writer: Writer<W>, pt_list: Vec<Point>) -> Result<(), Box<dyn Error>> {
    for point in pt_list {
        let formatted = format_point(&point);
        writer.write_record(&[formatted])?;
    }
    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let points = vec![
        Point {
            x: 1.0,
            y: 2.0,
            tag_color: "#FF808080".to_string(),
        },
        Point {
            x: 3.0,
            y: 4.0,
            tag_color: "#80FF8080".to_string(),
        },
    ];

    let file = File::create("output.csv")?;
    let mut writer = WriterBuilder::new().delimiter(b',').from_writer(file);

    save_points(writer, points)?;

    println!("Points saved to output.csv");
    Ok(())
}

#[test]
fn test_save_points() {
    let points = vec![
        Point {
            x: 1.0,
            y: 2.0,
            tag_color: "#FF808080".to_string(),
        },
        Point {
            x: 3.0,
            y: 4.0,
            tag_color: "#80FF8080".to_string(),
        },
    ];

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_owned();
    let file = File::create(&file_path).unwrap();

    let mut writer = WriterBuilder::new().delimiter(b',').from_writer(file);
    save_points(writer, points.clone()).unwrap();

    let file_contents = std::fs::read_to_string(&file_path).unwrap();
    let expected_contents = "\"(1.0, 2.0) #FF808080\"\n\"(3.0, 4.0) #80FF8080\"\n";
    assert_eq!(file_contents, expected_contents);
}

