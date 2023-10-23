use std::f32::consts::PI;
use csv::{ReaderBuilder, Writer, Trim};
use std::io::Read;
use std::fs::File;
use clap::{App, Arg};

struct Point {
    x: f32,
    y: f32,
}

struct PolarPoint {
    r: f32,
    t: f32,
}

fn to_polar(v: &[Point]) -> Vec<PolarPoint> {
    v.iter()
        .map(|point| {
            let r = (point.x * point.x + point.y * point.y).sqrt();
            let theta = (point.y.atan2(point.x) + 2.0 * PI) % (2.0 * PI); // Ensure angle is in the range [0, 2π)
            PolarPoint { r, t: theta }
        })
        .collect()
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
        let x: f32 = rec[0].parse().unwrap();
        let y: f32 = rec[1].parse().unwrap();
        out_list.push(Point{x, y});
        }
    }
    out_list
}

fn save_points_as_html<W: std::io::Write>(writer: W, pt_list: Vec<PolarPoint>) {
    let mut wtr = Writer::from_writer(writer);

    let html_header = r#"
        <html>
        <head>
            <style>
                table, td {
                    border: 1px solid #000000;
                    border-collapse: collapse;
                    padding: 5px;
                }
                table {
                    width: 100%;
                }
                th {
                    background-color: #f2f2f2;
                }
            </style>
        </head>
        <body>
            <table>
                <tr>
                    <th>Radius (r)</th>
                    <th>Theta (t)</th>
                </tr>
    "#;

    wtr.write_record(&[html_header.to_string()]).unwrap();

    for pt in pt_list {
        let row = format!(
            "<tr><td>{}</td><td>{}</td></tr>",
            pt.r.to_string(),
            pt.t.to_string()
        );
        wtr.write_record(&[row]).unwrap();
    }

    let html_footer = r#"
            </table>
        </body>
        </html>
    "#;

    wtr.write_record(&[html_footer.to_string()]).unwrap();

    wtr.flush().unwrap();
}



fn main() {
    let matches = App::new("Point_to_Polar")
        .version("1.0")
        .author("SaveSpoon")
        .about("Converting struct Point to PolarPoint")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("INPUT_FILE")
                .help("Sets the input CSV file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Sets the output CSV file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let input_name = matches.value_of("input").unwrap();
    let output_name = matches.value_of("output").unwrap();

    let point_list = load_pairs(File::open(input_name).unwrap());
    let polar_point = to_polar(&point_list);
    save_points_as_html(File::create(output_name).unwrap(), polar_point);
}
