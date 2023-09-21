const PI: f32 = 3.1416;
fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len()< 2 {
        panic!("No radius specified")
    }
    let r_arg = &args[1];
    let r: f32 = r_arg.parse().unwrap();
    
    let area: f32 = PI*r*r;

    print!("Area is {}", area);

    
}


