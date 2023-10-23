fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("No input celsius")
    }
    let c_arg = &args[1];
    let c : f32 = c_arg.parse().unwrap();

    let f:f32 = (9./5.)*c + 32.;
    
    print!("{}°C is {}°F", c, f )

}