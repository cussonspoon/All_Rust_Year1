use rand::Rng;
use csv::{ReaderBuilder, Writer};
use std::error::Error;
use std::fs::File;
use clap::{App, Arg};
use std::io::Read;

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
}

fn cal_average_area(layers: Vec<Layer>) -> Vec<Area> {
    let mut result: Vec<Area> = Vec::new();
    for layer in &layers {
        let mut total_area = 0.0;
        for circle in &layer.objects {
            let area = (circle.r as f32).powf(2.0) * 3.1416;
            total_area += area;
        }
        let average_area = if !layer.objects.is_empty() {
            total_area / layer.objects.len() as f32
        } else {
            0.0
        };
        result.push(Area {
            name: layer.name.clone(),
            area: average_area,
        });
    }
    result
}

fn cal_average_areas(layers: &[Layer]) -> Vec<Area> {
    let mut result: Vec<Area> = Vec::new();
    for layer in layers {
        let mut total_area = 0.0;
        for circle in &layer.objects {
            let area = (circle.r.abs() as f32).powi(2) * 3.1416;
            total_area += area;
        }
        let average_area = if !layer.objects.is_empty() {
            total_area / layer.objects.len() as f32
        } else {
            0.0
        };
        result.push(Area {
            name: layer.name.clone(),
            area: average_area,
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


fn save_areas<W: std::io::Write>(writer: W, areas: Vec<Area>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_writer(writer);

    for area in areas {
        let name = area.name;
        let area_value = area.area.to_string();
        wtr.write_record(&[name, area_value])?;
    }

    wtr.flush()?;
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
                .help("Sets the output CSV file name")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let input_name = matches.value_of("i").unwrap();
    let output_name = matches.value_of("o").unwrap();
    let point_list = load_layers(File::open(input_name).unwrap());
    let layer_list = cal_average_area(point_list);
    save_areas(File::create(output_name)?, layer_list)?;
    Ok(())
}
