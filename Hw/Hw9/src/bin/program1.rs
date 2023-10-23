use clap::{App, Arg};
use csv::Writer;
use rand::Rng;
use std::error::Error;
use std::fs::File;

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

fn gen_obj_layer_list<R: Rng>(rng: &mut R, n: i32) -> Vec<Layer> {
    let mut result2: Vec<Layer> = Vec::new();
    for i in 1..=n {
        let mut result: Vec<Circle> = Vec::new();
        let number_circle = rng.gen_range(20..=50);
        for _ in 0..number_circle {
            let x = rng.gen_range(-100..=100);
            let y = rng.gen_range(-100..=100);
            let r = rng.gen_range(-10..=10);
            result.push(Circle { x, y, r });
        }

        let color = format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255)
        );

        result2.push(Layer {
            name: format!("Layer {}", i),
            color,
            objects: result,
        });
    }

    result2
}

fn save_points<W: std::io::Write>(writer: W, layers: Vec<Layer>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_writer(writer);

    for layer in layers {
        let name = layer.name;
        let color = layer.color;
        let points: Vec<String> = layer.objects
            .iter()
            .map(|point| format!("{},{},{}", point.x, point.y, point.r))
            .collect();

        wtr.write_record(&[name, color, points.join(";")])?;
    }

    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Circle Generator")
        .arg(
            Arg::with_name("n")
                .short("n")
                .long("n")
                .value_name("N")
                .help("Sets the value of N")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output_name")
                .short("o")
                .long("output")
                .value_name("OUTPUT_NAME")
                .help("Sets the output CSV file name")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let n: i32 = matches.value_of("n").unwrap().parse()?;
    let output_name = matches.value_of("output_name").unwrap();

    let mut rng = rand::thread_rng();
    let layer_list = gen_obj_layer_list(&mut rng, n);

    save_points(File::create(output_name)?, layer_list)?;
    Ok(())
}
