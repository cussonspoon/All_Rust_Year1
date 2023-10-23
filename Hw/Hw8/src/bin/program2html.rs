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


fn to_cartesian(v: &[PolarPoint]) -> Vec<Point> {
    v.iter()
        .map(|polar_point| {
            let x = polar_point.r * polar_point.t.cos();
            let y = polar_point.r * polar_point.t.sin();
            Point { x, y }
        })
        .collect()
}

fn load_pairs<R: Read>(rdr: R) -> Vec<PolarPoint> {
    let mut reader= ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(rdr);
    let mut out_list = vec![];
    for record in reader.records() {
        if let Ok(rec) = record {
            let r: f32 = rec[0].parse().unwrap();
            let t: f32 = rec[1].parse().unwrap();
            out_list.push(PolarPoint { r, t });
        }
    }
    out_list
}


fn save_points_as_html<W: std::io::Write>(writer: W, pt_list: Vec<Point>) {
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
                    <th>x-value (x)</th>
                    <th>y-value (y)</th>
                </tr>
    "#;

    wtr.write_record(&[html_header.to_string()]).unwrap();

    for pt in pt_list {
        let row = format!(
            "<tr><td>{}</td><td>{}</td></tr>",
            pt.x.to_string(),
            pt.y.to_string()
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
    let matches = App::new("Polar_to_Point")
        .version("1.0")
        .author("SaveSpoon")
        .about("Converting struct PolarPoint to Point")
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
    let cartesian_point = to_cartesian(&point_list);
    save_points_as_html(File::create(output_name).unwrap(), cartesian_point);
}