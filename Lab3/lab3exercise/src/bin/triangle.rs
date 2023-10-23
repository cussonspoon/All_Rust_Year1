fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print!("")
    }
    let n_arg = &args[1];
    let n : i32 = n_arg.parse().unwrap();

    fn triangle(n: i32){
        for i in 0..n{
    for j in 0..i + 1{
     print!("*")}
     println!("");
    }
    }
triangle(n)


}

     
     
    
    // if args[1] == "1" {
    //     print!("*")
    // }
    // if args[1] == "2" {
    //     print!("*\n**")
    // }
    // if args[1] == "3" {
    //     print!("*\n**\n***")
    // }
    // if args[1] == "4" {
    //     print!("*\n**\n***\n****")
    // }
    // if args[1] == "5" {
    //     print!("*\n**\n***\n****\n*****")
    // }



