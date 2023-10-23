fn main(){
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        print!("Player 1 : N/A\nPlayer 2 : N/A")
    }
    else if args.len() == 3 {let name1: &String = &args[2];
        print!("Player 1 : {}\nPlayer 2 : N/A", name1);
    }
    else if args.len() >= 4 {let name2: &String = &args[3];
        let name1: &String = &args[2];
        print!("Player 1 : {}\nPlayer 2 : {}", name1, name2);   
    }
   
    


}