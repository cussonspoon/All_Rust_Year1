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
    points.sort_by(|a, b| {
        let x_cmp = a.0.cmp(&b.0);
        if x_cmp == std::cmp::Ordering::Equal {
            a.1.cmp(&b.1)
        } else {
            x_cmp
        }
    });

    println!("Sorted points in ascending order by x and y: {:?}", points);

    points.sort_by(|a, b| {
        let x_cmp = b.0.cmp(&a.0);
        if x_cmp == std::cmp::Ordering::Equal {
            b.1.cmp(&a.1)
        } else {
            x_cmp
        }
    });

    println!("Sorted points in descending order by x and y: {:?}", points);
}

