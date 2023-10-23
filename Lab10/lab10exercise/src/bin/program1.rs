use rand::Rng;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use csv::Writer;

struct RandConfig {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
}

struct Point {
    x: f32,
    y: f32,
}

struct Circle {
    center: Point,
    radius: f32,
}

enum Location {
    Inside(f32),
    Outside(f32),
}

fn gen_point_list<R: Rng>(
    rng: &mut R,
    cfg: RandConfig,
    n: i32,
) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    for _ in 0..n {
        let x = rng.gen_range(cfg.x_min..=cfg.x_max);
        let y = rng.gen_range(cfg.y_min..=cfg.y_max);
        result.push(Point { x, y });
    }
    result
}

fn locate_n_point(c: &Circle, pt_list: &[Point]) -> Vec<Location> {
    let mut result: Vec<Location> = Vec::new();

    for point in pt_list {
        let distance =
            ((point.x - c.center.x).powi(2) + (point.y - c.center.y).powi(2)).sqrt();

        let status = if distance <= c.radius {
            Location::Inside(distance)
        } else {
            Location::Outside(distance)
        };

        result.push(status);
    }

    result
}

fn generate_svg(
    locations: Vec<Location>,
    points: Vec<Point>,
    circle: Circle,
) -> Result<(), Box<dyn Error>> {
    let mut svg =
        String::from("<svg width=\"500\" height=\"500\" xmlns=\"http://www.w3.org/2000/svg\">\n");
    svg.push_str("<rect width=\"100%\" height=\"120%\" fill=\"#EEEEEE\" />\n");
    svg.push_str(&format!(
        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"black\" stroke-width=\"2\" fill=\"none\" />\n",
        circle.center.x, circle.center.y, circle.radius
    ));

    for (location, point) in locations.iter().zip(points.iter()) {
        let (color, cx, cy) = match location {
            Location::Inside(_) => ("green", point.x, point.y),
            Location::Outside(_) => ("red", point.x, point.y),
        };

        svg.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{}\" />\n",
            cx, cy, color
        ));
    }

    svg.push_str("</svg>");

    
    let mut file = File::create("Help.svg")?;
    file.write_all(svg.as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Point Locator")
        .arg(Arg::with_name("n")
            .short("n")
            .long("num-points")
            .value_name("N")
            .help("Sets the number of random points to generate")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("x_min")
            .long("x-min")
            .value_name("X_MIN")
            .help("Sets the minimum x-coordinate for random points")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("x_max")
            .long("x-max")
            .value_name("X_MAX")
            .help("Sets the maximum x-coordinate for random points")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("y_min")
            .long("y-min")
            .value_name("Y_MIN")
            .help("Sets the minimum y-coordinate for random points")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("y_max")
            .long("y-max")
            .value_name("Y_MAX")
            .help("Sets the maximum y-coordinate for random points")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("center_x")
            .long("center-x")
            .value_name("CENTER_X")
            .help("Sets the x-coordinate for the center of the circle")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("center_y")
            .long("center-y")
            .value_name("CENTER_Y")
            .help("Sets the y-coordinate for the center of the circle")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("radius")
            .long("radius")
            .value_name("RADIUS")
            .help("Sets the radius of the circle")
            .takes_value(true)
            .required(true))
        .get_matches();

    
    let num_points = matches.value_of("n").unwrap().parse::<i32>()?;
    let x_min = matches.value_of("x_min").unwrap().parse::<f32>()?;
    let x_max = matches.value_of("x_max").unwrap().parse::<f32>()?;
    let y_min = matches.value_of("y_min").unwrap().parse::<f32>()?;
    let y_max = matches.value_of("y_max").unwrap().parse::<f32>()?;
    let center_x = matches.value_of("center_x").unwrap().parse::<f32>()?;
    let center_y = matches.value_of("center_y").unwrap().parse::<f32>()?;
    let radius = matches.value_of("radius").unwrap().parse::<f32>()?;
    let mut rng = rand::thread_rng();

    let rand_config = RandConfig {
        x_min,
        x_max,
        y_min,
        y_max,
    };

    let point_list = gen_point_list(&mut rng, rand_config, num_points);
    let circle = Circle {
        center: Point { x: center_x, y: center_y },
        radius,
    };
    let results = locate_n_point(&circle, &point_list);
    generate_svg(results, point_list, circle)?;

    Ok(())
}

//cargo run --bin program1 -- --center-x 250 --center-y 250 --num-points 200 --radius 200 --x-max 500 --x-min 0 --y-max 500 --y-min 0
