use rand::Rng;

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Layer {
    name: String,
    color: String,
    points: Vec<Point>,
}

fn gen_layer<'a, R: Rng>(name: String, color: String, rng: &mut R) -> Layer{
    let mut result: Vec<Point> = Vec::new();
    let n = rng.gen_range(20..=50);
    for _ in 0..n {
        let x = rng.gen_range(-100..=100);
        let y = rng.gen_range(-100..=100);
        result.push(Point { x, y });
    }
    Layer {
        name,
        color,
        points: result,
    }
}

fn gen_layer_list<'a, R: Rng>(rng : &mut R, n : i32) -> Vec<Layer>{
    let mut result: Vec<Point> = Vec::new();
    let mut result2 : Vec<Layer> = Vec::new();
    let j = n+1;
    for i in 1..j {
        let x = rng.gen_range(-100..=100);
        let y = rng.gen_range(-100..=100);
        result.push(Point { x, y });
        let color = format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255)
        );
        result2.push(Layer{name : format!("Layer {}", i),color : color, points : result.clone()});
    }
 result2
}

fn main(){}
#[test]
fn test_gen_layer() {
    let mut rng = rand::thread_rng();
    let generated_layer_with_points = gen_layer("Layer with Points".to_string(), "Green".to_string(), &mut rng);
    let point_count = generated_layer_with_points.points.len();
    assert!(point_count >= 20 && point_count <= 50);

    for point in &generated_layer_with_points.points {
        assert!(point.x >= -100 && point.x <= 100);
        assert!(point.y >= -100 && point.y <= 100);
    }
}

#[test]
fn test_gen_layer_list() {
    let mut rng = rand::thread_rng();

    let expected_layer_count = 5;
    let generated_layer_list = gen_layer_list(&mut rng, expected_layer_count);
    for (i, layer) in generated_layer_list.iter().enumerate() {
        assert!(!layer.name.is_empty());
        assert_eq!(layer.points.len(), i + 1);
        assert_eq!(layer.color.len(), 9);
    }
}



