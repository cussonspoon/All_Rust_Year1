fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("No input score")
    }
    let x_args: &String = &args[1];
    let x: i32 = x_args.parse().unwrap();
    if x < 0{
        print!{"Invalid score"}
    }
    else if x >= 0 && x <= 49 {
        print!{"Failed with F"}
    }
    else if x >= 50  && x <= 60 {
        print!{"D"}
    }
    else if x >= 61  && x <= 70 {
        print!{"C"}
    }
    else if x >= 71  && x <= 80 {
        print!{"B"}
    }
    else if x >= 81  && x <= 94 {
        print!{"A"}
    }
    else if x >= 95  && x <= 100 {
        print!{"Excellent with A+"}
    }
    else{
        print!{"Invalid score"}
    } 
}
