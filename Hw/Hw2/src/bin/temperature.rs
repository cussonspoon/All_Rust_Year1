fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("No input farenheit")
    }
    let f_arg = &args[1];
    let f : f32 = f_arg.parse().unwrap();

    let c:f32 = (5./9.)*(f - 32.);
    
    print!("{}°F is {}°C", f, c )

}

