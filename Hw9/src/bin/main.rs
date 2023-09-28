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
#[derive(Clone)]
struct Area {
    name: String,
    area: f32,
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
fn cal_average_area(layers: Vec<Layer>) -> Vec<Area> {
    let mut result: Vec<Area> = Vec::new();
    for layer in &layers {
        let mut total_area = 0.0;
        for circle in &layer.objects {
            let area = (circle.r as f32).powf(2.0) * 3.1416;
            total_area += area;
        }
        let average_area = if !layer.objects.is_empty() {
            total_area / layer.objects.len() as f32
        } else {
            0.0
        };
        result.push(Area {
            name: layer.name.clone(),
            area: average_area,
        });
    }
    result
}

#[test]
fn test_gen_obj_layer_list() {
    let mut rng = rand::thread_rng();
    for n in 0..=10 {
        let layers = gen_obj_layer_list(&mut rng, n);
        for (i, layer) in layers.iter().enumerate() {
            assert!(layer.name.starts_with("Layer "));
            assert!(layer.color.starts_with("#"));
            assert_eq!(layer.color.len(), 9);
            for circle in &layer.objects {
                assert!(circle.x >= -100 && circle.x <= 100);
                assert!(circle.y >= -100 && circle.y <= 100);
                assert!(circle.r >= -10 && circle.r <= 10);
            }
        }
    }
}

#[test]
fn test_cal_average_area() {
    let layers = vec![
        Layer {
            name: "Layer 1".to_string(),
            color: "#FF0000FF".to_string(),
            objects: vec![Circle { x: 0, y: 0, r: 5 }, Circle { x: 0, y: 0, r: 10 }],
        },
        Layer {
            name: "Layer 2".to_string(),
            color: "#00FF00FF".to_string(),
            objects: vec![
                Circle { x: 0, y: 0, r: 3 },
                Circle { x: 0, y: 0, r: 8 },
                Circle { x: 0, y: 0, r: 12 },
            ],
        },
    ];

    let average_areas = cal_average_area(layers.clone());
    assert_eq!(average_areas.len(), 2);
    assert_eq!(average_areas[0].name, "Layer 1");
    assert_eq!(
        average_areas[0].area,
        (5.0 * 5.0 * 3.1416 + 10.0 * 10.0 * 3.1416) / 2.0
    );

    assert_eq!(average_areas[1].name, "Layer 2");
    assert_eq!(
        average_areas[1].area,
        (3.0 * 3.0 * 3.1416 + 8.0 * 8.0 * 3.1416 + 12.0 * 12.0 * 3.1416) / 3.0
    );
}

fn main() {}
