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
    let mut asen = result.clone();
    asen.sort_by(|a,b | a.cmp(b));
    let mut decend = result.clone();
    decend.sort_by(|a, b| b.cmp(a));
    println!("Ascending order: {:?}", asen);
    println!("Descending order: {:?}", decend);
}
