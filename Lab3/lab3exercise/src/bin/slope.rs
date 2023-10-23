fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("No input coordinates")
    }
    let x1_arg = &args[1];
    let x1 : f32 = x1_arg.parse().unwrap();
    let y1_arg = &args[2];
    let y1 : f32 = y1_arg.parse().unwrap();
    let x2_arg = &args[3];
    let x2 : f32 = x2_arg.parse().unwrap();
    let y2_arg = &args[4];
    let y2 : f32 = y2_arg.parse().unwrap();

    let m:f32 = (y2-y1)/(x2-x1);
    
    print!("m = {}", m)
    
}