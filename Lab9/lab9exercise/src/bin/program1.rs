use rand::Rng;
use csv::Writer;
use std::fs::File;
use clap::{ App, Arg };
use std::error::Error;

struct Point {
    x: i32,
    y: i32,
}

struct Layer {
    name: String,
    color: String,
    points: Vec<Point>,
}

fn gen_layer_list<R: Rng>(rng: &mut R, n: i32) -> Vec<Layer> {
    let mut result2: Vec<Layer> = Vec::new();
    let j = n + 1;

    for i in 1..j {
        let mut result: Vec<Point> = Vec::new();
        let k = rng.gen_range(0..=n);
        for _ in 0..k {
            let x = rng.gen_range(-100..=100);
            let y = rng.gen_range(-100..=100);
            result.push(Point { x, y });
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
            points: result,
        });
    }

    result2
}

fn save_points<W: std::io::Write>(writer: W, layers: Vec<Layer>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_writer(writer);

    for layer in layers {
        let name = layer.name;
        let color = layer.color;
        let points: Vec<String> = layer.points
            .iter()
            .map(|point| format!("{},{}", point.x, point.y))
            .collect();

        wtr.write_record(&[name, color, points.join(",")])?;
    }

    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Create Layer and colors")
        .version("1.0")
        .author("SaveSpoon")
        .about("Creating layers and colors")
        .arg(
            Arg::with_name("Layers")
                .short("l")
                .long("Layers")
                .value_name("n")
                .help("Define how many layers the output should have")
                .required(true)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Sets the output CSV file")
                .required(true)
                .takes_value(true)
        )
        .get_matches();

    let n_arg = matches.value_of("Layers").unwrap();
    let n: i32 = n_arg.parse()?;
    let output_name = matches.value_of("output").unwrap();

    let mut rng = rand::thread_rng();
    let layer_list = gen_layer_list(&mut rng, n);
    save_points(File::create(output_name)?, layer_list)?;

    Ok(())
}
