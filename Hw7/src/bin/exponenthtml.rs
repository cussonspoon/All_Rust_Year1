fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("No or not enough value provided");
        return;
    }
    let x_args: &String = &args[1];
    let x: f32 = x_args.parse().unwrap();
    let y_args: &String = &args[2];
    let y: f32 = y_args.parse().unwrap();
    let z_args: &String = &args[3];
    let z: i32 = z_args.parse().unwrap();
    if z == 0{
        println!("The interval cannot be 0 (Infinite loop will occur)");
        return;
    }
    println!("<style>");
    println!("table, td {{");
    println!("   border: 1px solid #000000;\n   border-collapse: collapse;\n}}");
    println!("</style>\n");
    println!("<table>");
    println!("   <tr>");
    println!("       <th>x</th>");
    println!("       <th>x^2</th>");
    println!("       <th>x^3</th>");
    println!("   </tr>");
    if x < y {
        let mut m = x;
        while m <= y {
            println!("   <tr>");
            println! {"       <td>{:.2}</td>\n       <td>{:.2}</td>\n       <td>{:.2}</td>", m, m*m, m*m*m}
            println!("   </tr>");
            m += z as f32;
        }
        println!("</table>")
    }
    if x > y {
        let mut n = x;
        while n >= y {
            println!("   <tr>");
            println! {"       <td>{:.2}</td>\n       <td>{:.2}</td>\n       <td>{:.2}</td>", n, n*n, n*n*n}
            println!("   </tr>");
            n -= z as f32;
        }
        println!("</table>")
    }
}
