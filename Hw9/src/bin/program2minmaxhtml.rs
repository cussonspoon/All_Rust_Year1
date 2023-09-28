use rand::Rng;
use csv::{ReaderBuilder, Writer};
use std::error::Error;
use std::fs::File;
use clap::{App, Arg};
use std::io::Read;
use std::fmt::Write;

#[derive(Clone)]
struct Circle {
    x: i32,
    y: i32,
    r: i32,
}

#[derive(Clone)]
struct Layer {
    name: String,
    color: String,
    objects: Vec<Circle>,
}

#[derive(Clone)]
struct Area {
    name: String,
    area: f32,
    min_area: f32,
    max_area: f32,
}

fn cal_average_area(layers: Vec<Layer>) -> Vec<Area> {
    let mut result: Vec<Area> = Vec::new();
    for layer in &layers {
        let mut total_area = 0.0;
        let mut min_area = f32::MAX;
        let mut max_area = f32::MIN;
        
        for circle in &layer.objects {
            let area = (circle.r as f32).powf(2.0) * 3.1416;
            total_area += area;
            min_area = min_area.min(area);
            max_area = max_area.max(area);
        }
        
        let average_area = if !layer.objects.is_empty() {
            total_area / layer.objects.len() as f32
        } else {
            0.0
        };
        
        result.push(Area {
            name: layer.name.clone(),
            area: average_area,
            min_area,
            max_area,
        });
    }
    result
}

fn load_layers<R: Read>(rdr: R) -> Vec<Layer> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .from_reader(rdr);
    let mut out_list = vec![];

    for record in reader.records() {
        if let Ok(rec) = record {
            let name: String = rec[0].parse().unwrap();
            let color: String = rec[1].parse().unwrap();
            let circles_str: String = rec[2].to_string();

            let mut objects = Vec::new();


            for circle_str in circles_str.split(';') {
                let xyz: Vec<&str> = circle_str.split(',').collect();
                if xyz.len() == 3 {
                    let x = xyz[0].parse().unwrap();
                    let y = xyz[1].parse().unwrap();
                    let r = xyz[2].parse().unwrap();
                    let circle = Circle { x, y, r };
                    objects.push(circle);
                }
            }

            out_list.push(Layer {
                name,
                color,
                objects,
            });
        }
    }

    out_list
}

fn save_areas_as_html(output_file: &str, areas: Vec<Area>) -> Result<(), Box<dyn Error>> {
    let mut html_content = String::new();

    html_content.push_str("<html><head><title>Layer Data</title></head><body>");
    html_content.push_str("<h1>Layer Data</h1>");
    
    html_content.push_str("<style>");
    html_content.push_str("table {border-collapse: collapse; width: 50%; margin: 20px auto;}");
    html_content.push_str("th, td {border: 1px solid black; padding: 8px; text-align: left;}");
    html_content.push_str("th {background-color: #f2f2f2;}");
    html_content.push_str("</style>");
    
    html_content.push_str("<table>");
    html_content.push_str("<tr><th>Layer</th><th>Average Area</th><th>Min Area</th><th>Max Area</th></tr>");

    for area in areas {
        let row = format!(
            "<tr><td>{}</td><td>{:.2}</td><td>{:.2}</td><td>{:.2}</td></tr>",
            area.name, area.area, area.min_area, area.max_area
        );
        html_content.push_str(&row);
    }

    html_content.push_str("</table>");
    html_content.push_str("</body></html>");


    std::fs::write(output_file, html_content)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Circle Generator")
        .arg(
            Arg::with_name("i")
                .short("i")
                .long("i")
                .value_name("INPUT_FILE")
                .help("Name of the input file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("o")
                .short("o")
                .long("o")
                .value_name("OUTPUT_NAME")
                .help("Sets the output HTML file name")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let input_name = matches.value_of("i").unwrap();
    let output_name = matches.value_of("o").unwrap();
    let point_list = load_layers(File::open(input_name).unwrap());
    let layer_list = cal_average_area(point_list);


    let output_name_str = output_name;


    save_areas_as_html(output_name_str, layer_list)?;

    Ok(())
}
