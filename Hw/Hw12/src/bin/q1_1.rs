#[derive(Debug)]
enum Shape {
    Circle(i32, i32, i32),
    Rectangle(i32, i32, i32, i32),
    Triangle((i32, i32), (i32, i32), (i32, i32))
}

impl Shape {
    fn rep_string(&self) -> String {
        match self {
            Shape::Circle(x, y, r) => {
                let f = format!("<Circle: {}, {}, {}>", x, y, r);
                f
            }
            Shape::Rectangle(x, y, w, h) => {
                let f = format!("<Rectangle: {}, {}, {}, {}>", x, y, w, h);
                f
            }
            Shape::Triangle((x1, y1),(x2, y2) ,(x3, y3) ) =>{
                let f = format!("<Triangle: ({}, {}), ({}, {}), ({}, {})>", x1, y1, x2, y2, x3, y3);
                f
            }
        }
    }

    fn area(&self) -> f32 {
        match self {
            Shape::Circle(x, y, r) => {
                let a = 3.1416 * (*r as f32) * (*r as f32);
                a
            }
            Shape::Rectangle(x, y, w, h) => {
                let a = w * h;
                a as f32
            }
            Shape::Triangle((x1, y1),(x2, y2) ,(x3, y3) ) =>{
                let a = 0.5 * ((x1 - x3)*(y2 - y1) - (x1 - x2)*(y3 - y1)) as f32;
                a
        }
    }
    }
}

const INPUT_SHAPES: &[Shape] = &[
    Shape::Circle(0, 0, 1),
    Shape::Circle(50, 50, 15),
    Shape::Rectangle(40, 40, 20, 20),
    Shape::Rectangle(10, 40, 15, 10),
    Shape::Triangle((10, 20), (30, 40), (15, 25)),
    Shape::Triangle((15, 25), (35, 45), (20, 40)),
];
const EXPECTED: &[&str] = &[
    "<Circle: 0, 0, 1>, area: 3.14",
    "<Circle: 50, 50, 15>, area: 706.86",
    "<Rectangle: 40, 40, 20, 20>, area: 400.00",
    "<Rectangle: 10, 40, 15, 10>, area: 150.00",
    "<Triangle: (10, 20), (30, 40), (15, 25)>, area: 0.00",
    "<Triangle: (15, 25), (35, 45), (20, 40)>, area: 100.00",
];
#[test]
fn test_shapes() {
    let input_list = INPUT_SHAPES;
    let shape_list = input_list.clone();
    let omap = shape_list.iter().map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED);
}

fn main() {print!("HELLOWORLD")}