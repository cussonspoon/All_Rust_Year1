fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("No input Fahr")
    }
    let x_args: &String = &args[1];
    let x: i32 = x_args.parse().unwrap();
    let y_args: &String = &args[2];
    let y: i32 = y_args.parse().unwrap();
    let z_args: &String = &args[3];
    let z: i32 = z_args.parse().unwrap();
    // x is the starting number
    // y is the ending number
    // z is the interval of the table

    if x < y { // ex. 0..300
        println!("Fahr Celcius");
        let mut m = x;
        while m <= y {
            let b : f32 = (5./9.)*(m as f32 - 32.);
            println!{"{}°F {:.1}°C", m, b}
            m += z;
        }
    }
    if x > y { // ex. 300..0
        println!("Fahr Celcius");
        let mut n = x;
        while n >= y {
            let c : f32 = (5./9.)*(n as f32 - 32.);
            println!{"{}°F {:.1}°C", n, c}
            n -= z;
        }
    }
}
//ALTERNATIVE
    // if x < y { // ex. 0..300
    //     println!("Fahr Celcius");
    //     for x in x..y+1{
    //         let f: i32 = x*z;
    //         let c : f32 = (5./9.)*(f as f32 - 32.) ;
    //         println!{"{} {:.1}", f, c}
    //         if f >= y{
    //             break
    //         }        
    //     }
    //  }











