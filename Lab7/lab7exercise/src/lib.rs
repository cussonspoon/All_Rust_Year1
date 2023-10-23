use rand::Rng;

fn filter_numbers(v: &[f32]) -> Vec<f32> {
    let mut result: Vec<f32> = Vec::new();
    if v.is_empty() {
        return result;
    }
    let mut iter = v.iter();
    while let Some(x) = iter.next() {
        if x >= &-1.0 && x <= &1.0 {
            result.push(*x);
        }
    }
    result
}

fn gen_number<R: Rng>(rng: &mut R, n: i64) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    for _ in 0..n {
        let num: f64 = rng.gen_range(-10.0..=10.0);
        result.push(num);
    }
    result
}

fn filter_points(v: &[(f32, f32)]) -> Vec<(f32, f32)> {
    let mut result: Vec<(f32, f32)> = Vec::new();
    if v.is_empty() {
        return result;
    }
    let mut iter = v.iter();
    while let Some((x, y)) = iter.next() {
        if (x + y).sqrt() <= 1.0 {
            result.push((*x, *y));
        }
    }
    result
}

fn gen_points<R: Rng>(rng: &mut R, n: i64) -> Vec<(f64, f64)> {
    let mut result: Vec<(f64, f64)> = Vec::new();
    for _ in 0..n {
        let num: f64 = rng.gen_range(-1.0..=1.0);
        let num2 : f64 = rng.gen_range(-1.0..=1.0);
        result.push((num, num2));
    }
    result
}


#[test]
fn test_filter_numbers() {
    assert_eq!(filter_numbers(&[]), []);
    assert_eq!(filter_numbers(&[2.0, 3.0, 5.0]), []);
    assert_eq!(filter_numbers(&[-2.0, -1.5, 0.0, 1.5, 2.0]), [0.0]);
    assert_eq!(
        filter_numbers(&[-2.0, -1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1.5, 2.0]),
        [-1.0, -0.5, 0.0, 0.5, 1.0]
    );
}

#[test]
fn test_gen_number() {
    let mut rng = rand::thread_rng();

    let generated_empty = gen_number(&mut rng, 0);
    assert_eq!(generated_empty.len(), 0);

    let generated_positive = gen_number(&mut rng, 5);
    assert_eq!(generated_positive.len(), 5);
    for &num in &generated_positive {
        assert!(num >= -10.0 && num <= 10.0);
    }

    let generated_negative = gen_number(&mut rng, -5);
    assert_eq!(generated_negative.len(), 0);
}

#[test]
fn test_filter_points() {
    assert_eq!(filter_points(&[]), []);
    assert_eq!(filter_points(&[(2.0, 3.0), (5.0, 6.0)]), []);
    assert_eq!(filter_points(&[(-2.0, -2.0), (-1.5, 1.5), (0.0, 0.0), (1.5, -1.5), (2.0, 2.0)]), [(-1.5, 1.5), (0.0, 0.0), (1.5, -1.5)]);
    assert_eq!(
        filter_points(&[(-2.0, -2.0), (-1.5, -1.5), (-1.0, -1.0), (-0.5, -0.5), (0.0, 0.0), (0.5, 0.5), (1.0, 1.0), (1.5, 1.5), (2.0, 2.0)]),
        [(0.0, 0.0), (0.5, 0.5)]
    );
}

#[test]
fn test_gen_points() {
    let mut rng = rand::thread_rng();

    let generated_empty = gen_points(&mut rng, 0);
    assert_eq!(generated_empty.len(), 0);

    let generated_positive = gen_points(&mut rng, 5);
    assert_eq!(generated_positive.len(), 5);
    for &(x, y) in &generated_positive {
        assert!(x >= -1.0 && x <= 1.0);
        assert!(y >= -1.0 && y <= 1.0);
    }

    let generated_negative = gen_points(&mut rng, -5);
    assert_eq!(generated_negative.len(), 0);
}