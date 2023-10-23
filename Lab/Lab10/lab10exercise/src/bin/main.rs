use rand::Rng;
use csv::Writer;
use std::fs::File;
use std::cell::RefCell;
use clap::{ App, Arg };
use std::error::Error;
use std::rc::Rc;

struct  RandConfig{
    x_min : f32,
    x_max : f32,
    y_min : f32,
    y_max : f32
}

struct Point{
    x : f32,
    y : f32
}

struct Circle{
    center : Point,
    radius : f32,
}

enum Location{
    Inside(f32),
    Outside(f32),
}


fn gen_point_list<R: Rng>(rng: Rc<RefCell<R>>, cfg: RandConfig, n: i32) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    for _ in 0..=n {
        let x = {
            let mut rng_mut = rng.borrow_mut();
            rng_mut.gen_range(cfg.x_min..=cfg.x_max)
        };
        let y = {
            let mut rng_mut = rng.borrow_mut();
            rng_mut.gen_range(cfg.y_min..=cfg.y_max)
        };
        result.push(Point { x, y });
    }
    result
}

fn locate_n_point(c: &Circle, pt_list: &[Point]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for point in pt_list {
        let distance = ((point.x - c.center.x).powi(2) + (point.y - c.center.y).powi(2)).sqrt();

        let status = if distance <= c.radius {
            Location::Inside(distance)
        } else {
            Location::Outside(distance)
        };

        let formatted_output = match status {
            Location::Inside(dist) => format!("Inside((Point {{ x: {:.2}, y: {:.2} }}, {:.2}))", point.x, point.y, dist),
            Location::Outside(dist) => format!("Outside((Point {{ x: {:.2}, y: {:.2} }}, {:.2}))", point.x, point.y, dist),
        };
        result.push(formatted_output);
    }

    result
}
#[test]
    fn test_locate_n_point_inside() {
        let circle = Circle {
            center: Point { x: 0.0, y: 0.0 },
            radius: 2.0,
        };
        let point_list = vec![
            Point { x: 1.0, y: 1.0 },
            Point { x: -1.0, y: -1.0 },
        ];
        let result = locate_n_point(&circle, &point_list);
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            "Inside((Point { x: 1.00, y: 1.00 }, 1.41))"
        );
        assert_eq!(
            result[1],
            "Inside((Point { x: -1.00, y: -1.00 }, 1.41))"
        );
    }

    #[test]
    fn test_locate_n_point_outside() {
        let circle = Circle {
            center: Point { x: 0.0, y: 0.0 },
            radius: 1.0,
        };
        let point_list = vec![
            Point { x: 2.0, y: 2.0 },
            Point { x: -2.0, y: -2.0 },
        ];
        let result = locate_n_point(&circle, &point_list);
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            "Outside((Point { x: 2.00, y: 2.00 }, 2.83))"
        );
        assert_eq!(
            result[1],
            "Outside((Point { x: -2.00, y: -2.00 }, 2.83))"
        );
    }

fn main(){}