fn main() {
    let mut points = Vec::new();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("No or not enough argument provided");
        return;
    }

    let mut i = 1;
    while i < args.len() {
        let x = match args[i].parse::<i32>() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Error parsing x-coordinate: {}", &args[i]);
                return;
            }
        };

        let y = match args.get(i + 1).map(|s| s.parse::<i32>()) {
            Some(Ok(parsed)) => parsed,
            Some(Err(_)) => {
                eprintln!("Error parsing y-coordinate: {}", args[i + 1]);
                return;
            }
            None => {
                i += 2;
                continue;
            }
        };

        points.push((x, y));

        i += 2;
    }

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            if points[i].0 > points[j].0 || (points[i].0 == points[j].0 && points[i].1 > points[j].1) {
                points.swap(i, j);
            }
        }
    }

    println!("Sorted points in ascending order by x and y: {:?}", points);

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            if points[i].0 < points[j].0 || (points[i].0 == points[j].0 && points[i].1 < points[j].1) {
                points.swap(i, j);
            }
        }
    }

    println!("Sorted points in descending order by x and y: {:?}", points);
}
