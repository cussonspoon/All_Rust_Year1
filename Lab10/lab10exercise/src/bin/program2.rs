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

struct Circle2{
    center : Point,
    radius : f32

}

struct Bound{
    circle1 : Circle,
    circle2 : Circle2,
}
enum Location {
    InsideCircle1(f32),
    InsideCircle2(f32),
    InsideBothCircles(f32, f32),
    OutsideBothCircles(f32, f32),
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

fn locate_n_point2(b: &Bound, pt_list: &[Point]) -> Vec<Location> {
    let mut result: Vec<Location> = Vec::new();

    for point in pt_list {
        let distance1 =
            ((point.x - b.circle1.center.x).powi(2) + (point.y - b.circle1.center.y).powi(2)).sqrt();
        let distance2 =
            ((point.x - b.circle2.center.x).powi(2) + (point.y - b.circle2.center.y).powi(2)).sqrt();

        let status = if distance1 <= b.circle1.radius {
            if distance2 <= b.circle2.radius {
                Location::InsideBothCircles(distance1, distance2)
            } else {
                Location::InsideCircle1(distance1)
            }
        } else if distance2 <= b.circle2.radius {
            Location::InsideCircle2(distance2)
        } else {
            Location::OutsideBothCircles(distance1, distance2)
        };

        result.push(status);
    }

    result
}


fn generate_svg(
    locations: Vec<Location>,
    points: Vec<Point>,
    b : Bound,
) -> Result<(), Box<dyn Error>> {
    let mut svg =
        String::from("<svg width=\"500\" height=\"500\" xmlns=\"http://www.w3.org/2000/svg\">\n");
    svg.push_str("<rect width=\"100%\" height=\"120%\" fill=\"#EEEEEE\" />\n");
    svg.push_str(&format!(
        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"black\" stroke-width=\"2\" fill=\"none\" />\n",
        b.circle1.center.x, b.circle1.center.y, b.circle1.radius
    ));
    svg.push_str(&format!(
        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"black\" stroke-width=\"2\" fill=\"none\" />\n",
        b.circle2.center.x, b.circle2.center.y, b.circle2.radius
    ));

    for (location, point) in locations.iter().zip(points.iter()) {
        let (color, cx, cy) = match location {
            Location::InsideBothCircles(_,_) => ("green", point.x, point.y),
            Location::OutsideBothCircles(_,_) => ("red", point.x, point.y),
            Location::InsideCircle1(_) => ("yellow", point.x, point.y),
            Location::InsideCircle2(_) => ("purple", point.x, point.y)
        };

        svg.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{}\" />\n",
            cx, cy, color
        ));
    }

    svg.push_str("</svg>");

    
    let mut file = File::create("Help2.svg")?;
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
        .arg(Arg::with_name("center_x1")
            .long("center-x1")
            .value_name("CENTER_X1")
            .help("Sets the x-coordinate for the center of the circle1")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("center_y1")
            .long("center-y1")
            .value_name("CENTER_Y1")
            .help("Sets the y-coordinate for the center of the circle1")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("radius1")
            .long("radius1")
            .value_name("RADIUS1")
            .help("Sets the radius of the circle1")
            .takes_value(true)
            .required(true))
            .arg(Arg::with_name("center_x2")
            .long("center-x2")
            .value_name("CENTER_X2")
            .help("Sets the x-coordinate for the center of the circle2")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("center_y2")
            .long("center-y2")
            .value_name("CENTER_Y2")
            .help("Sets the y-coordinate for the center of the circle2")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("radius2")
            .long("radius2")
            .value_name("RADIUS2")
            .help("Sets the radius of the circle2")
            .takes_value(true)
            .required(true))
        .get_matches();

    
    let num_points = matches.value_of("n").unwrap().parse::<i32>()?;
    let x_min = matches.value_of("x_min").unwrap().parse::<f32>()?;
    let x_max = matches.value_of("x_max").unwrap().parse::<f32>()?;
    let y_min = matches.value_of("y_min").unwrap().parse::<f32>()?;
    let y_max = matches.value_of("y_max").unwrap().parse::<f32>()?;
    let center_x1 = matches.value_of("center_x1").unwrap().parse::<f32>()?;
    let center_y1 = matches.value_of("center_y1").unwrap().parse::<f32>()?;
    let radius1 = matches.value_of("radius1").unwrap().parse::<f32>()?;
    let center_x2 = matches.value_of("center_x2").unwrap().parse::<f32>()?;
    let center_y2 = matches.value_of("center_y2").unwrap().parse::<f32>()?;
    let radius2 = matches.value_of("radius2").unwrap().parse::<f32>()?;
    let mut rng = rand::thread_rng();

    let rand_config = RandConfig {
        x_min,
        x_max,
        y_min,
        y_max,
    };

    let point_list = gen_point_list(&mut rng, rand_config, num_points);
    let circle1 = Circle {
        center: Point { x: center_x1, y: center_y1 },
        radius : radius1,
    };
    let circle2 = Circle2{
        center : Point{x : center_x2, y: center_y2},
        radius : radius2,
    };
    let b = Bound{
        circle1 : circle1,
        circle2 : circle2,
    };


    let results = locate_n_point2(&b, &point_list);
    generate_svg(results, point_list, b)?;

    Ok(())
}

//cargo run --bin program2 -- --center-x1 150 --center-x2 350 --center-y1 150 --center-y2 150 --num-points 200 --radius1 150 --radius2 150 --x-max 500 --x-min 0 --y-max 500 --y-min 0