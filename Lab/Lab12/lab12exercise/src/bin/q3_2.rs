
trait Shape {
    fn rep_string(&self) -> String;
    fn area(&self) -> f64;
    fn clone_shape(&self) -> Box<dyn Shape>;
}


struct Circle {
    x: i32,
    y: i32,
    r: i32,
}

struct Rectangle {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Box<dyn Shape> {
        self.clone_shape()
    }
}

impl Shape for Circle {
    fn rep_string(&self) -> String {
        format!("<Circle: {}, {}, {}>", self.x, self.y, self.r)
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.r as f64) * (self.r as f64)
    }

    fn clone_shape(&self) -> Box<dyn Shape> {
        Box::new(Circle { x: self.x, y: self.y, r: self.r })
    }
}

impl Shape for Rectangle {
    fn rep_string(&self) -> String {
        format!("<Rectangle: {}, {}, {}, {}>", self.x, self.y, self.w, self.h)
    }

    fn area(&self) -> f64 {
        (self.w as f64) * (self.h as f64)
    }

    fn clone_shape(&self) -> Box<dyn Shape> {
        Box::new(Rectangle { x: self.x, y: self.y, w: self.w, h: self.h })
    }
}

impl Circle {
    fn new(x: i32, y: i32, r: i32) -> Box<dyn Shape> {
        Box::new(Circle { x, y, r })
    }
}

impl Rectangle {
    fn new(x: i32, y: i32, w: i32, h: i32) -> Box<dyn Shape> {
        Box::new(Rectangle { x, y, w, h })
    }
}


fn input_shape_list() -> Vec<Box<dyn Shape>> {
    vec![
        Circle::new(0, 0, 1),
        Circle::new(50, 50, 15),
        Rectangle::new(40, 40, 20, 20),
        Rectangle::new(10, 40, 15, 10)
    ]
}

const EXPECTED_001: &[&str] = &[
    "<Circle: 0, 0, 1>",
    "<Circle: 50, 50, 15>",
    "<Rectangle: 40, 40, 20, 20>",
    "<Rectangle: 10, 40, 15, 10>",
];

const EXPECTED_002: &[&str] = &[
"<Circle: 0, 0, 1>, area: 3.14",
"<Circle: 50, 50, 15>, area: 706.86",
"<Rectangle: 40, 40, 20, 20>, area: 400.00",
"<Rectangle: 10, 40, 15, 10>, area: 150.00"
];


#[test]
fn test_shapes_001() {
    let shape_list = input_shape_list();
    let omap = shape_list.iter().map(|s| s.rep_string());
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_001);
}

#[test]
fn test_shapes_002() {
let shape_list = input_shape_list();
let omap = shape_list.iter().map(
|s| format!("{}, area: {:.2}", s.rep_string(), s.area()) );
let output: Vec<_> = omap.collect();
assert_eq!(output, EXPECTED_002);
}

#[test]
fn test_shapes_003() {
let input_list = input_shape_list();
let shape_list = input_list.clone();
let omap = shape_list.iter().map(
|s| format!("{}, area: {:.2}", s.rep_string(), s.area()) );
let output: Vec<_> = omap.collect();
assert_eq!(output, EXPECTED_002);
}

fn main() {}