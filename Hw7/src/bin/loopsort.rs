fn main() {
    let mut result: Vec<i32> = Vec::new();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("No argument provided");
        return;
    }

    for arg in args.iter().skip(1) {
        match arg.parse::<i32>() {
            Ok(num) => result.push(num),
            Err(e) => {
                println!("Error parsing argument '{}': {}", arg, e);
                return;
            }
        }
    }
    let mut swapped;

    loop {
        swapped = false;
        for i in 1..result.len() {
            if result[i - 1] > result[i] {
                result.swap(i - 1, i);
                swapped = true;
            }
        }
        
        if !swapped {
            break;
        }
    }
    println!("Ascending order: {:?}", result);

    loop {
        swapped = false;
        for i in 1..result.len() {
            if result[i - 1] < result[i] {
                result.swap(i - 1, i);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    println!("Descending order: {:?}", result);
}
