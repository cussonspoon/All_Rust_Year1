fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print!("")
    }
    else{
    let n_arg = &args[1];
    let n : i32 = n_arg.parse().unwrap();
    fn triangle(n: i32){
        let mut i = n;
        while i > 0 {
            let mut x = i;
            while x <= n{
                print!("*");
                x += 1;
            } 
            println!("");
            i -= 1
        }
    }
    fn triangle2(n: i32){
        let mut i = n;
        while i-1 > 0 {
            let mut x = i;
            while x-1 > 0{
                print!("*");
                x -= 1
            } 
            println!("");
            i -= 1
        }

    }
triangle(n);
triangle2(n);
}
}

 // fn triangle(n: i32){
    //     for i in 0..n{
    // for j in 0..i + 1{
    //  print!("*")}
    //  println!("")
    
    // }
    // }