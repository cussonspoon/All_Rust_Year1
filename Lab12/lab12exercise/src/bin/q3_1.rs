#[derive(Debug)]
enum Shape {
    Circle(i32, i32, i32),
    Rectangle(i32, i32, i32, i32),
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
        }
    }
}

const INPUT_SHAPES: &[Shape] = &[
    Shape::Circle(0, 0, 1),
    Shape::Circle(50, 50, 15),
    Shape::Rectangle(40, 40, 20, 20),
    Shape::Rectangle(10, 40, 15, 10),
];
const EXPECTED: &[&str] = &[
    "<Circle: 0, 0, 1>, area: 3.14",
    "<Circle: 50, 50, 15>, area: 706.86",
    "<Rectangle: 40, 40, 20, 20>, area: 400.00",
    "<Rectangle: 10, 40, 15, 10>, area: 150.00",
];
#[test]
fn test_shapes() {
    let input_list = INPUT_SHAPES;
    let shape_list = input_list.clone();
    let omap = shape_list.iter().map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED);
}

fn main() {}
