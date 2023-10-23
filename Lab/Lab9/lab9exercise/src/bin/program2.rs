use rand::Rng;
use std::{fs::File, io::Write};
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

fn gen_layer<'a, R: Rng>(name: String, color: String, rng: &mut R) -> Layer {
    let mut result: Vec<Point> = Vec::new();
    let n = rng.gen_range(20..=50);
    for _ in 0..n {
        let x = rng.gen_range(-250..=250);
        let y = rng.gen_range(-250..=250);
        result.push(Point { x, y });
    }
    Layer {
        name,
        color,
        points: result,
    }
}

fn generate_svg(layers: &Vec<Layer>) -> String {
    let mut svg = String::from("<svg width=\"500\" height=\"500\" xmlns=\"http://www.w3.org/2000/svg\">\n");
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#EEEEEE\" />\n");
    
    for layer in layers {
        svg.push_str(&format!("<circle cx=\"250\" cy=\"250\" r=\"250\" stroke=\"black\" stroke-width=\"2\" fill=\"none\" />\n"));
        for point in &layer.points {
            svg.push_str(&format!("<circle cx=\"{}\" cy=\"{}\" r=\"10\" fill=\"{}\" />\n", point.x + 250, point.y + 250, layer.color));
        }
    }

    svg.push_str("</svg>");
    svg
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Create Layers and Colors")
        .version("1.0")
        .author("SaveSpoon")
        .about("Creating layers and colors")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Sets the output SVG file")
                .required(true)
                .takes_value(true)
        )
        .get_matches();

    let output_name = matches.value_of("output").unwrap();

    let mut rng = rand::thread_rng();
    let mut layer_list = Vec::new();

    let layer = gen_layer("Layer1".to_string(), "#A1B2C3D4".to_string(), &mut rng);
    layer_list.push(layer);


    let svg_content = generate_svg(&layer_list);
    let mut output_file = File::create(output_name)?;
    output_file.write_all(svg_content.as_bytes())?;

    Ok(())
}
