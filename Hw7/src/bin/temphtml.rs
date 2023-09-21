fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        println!("Not enough information provided");
        return;
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
    println!("       <th>Fahr</th>");
    println!("       <th>Celcius</th>");
    println!("   </tr>");

    if x < y {
        // ex. 0..300
        let mut m = x;
        while m <= y {
            println!("   <tr>");
            let b: f32 = (5. / 9.) * (m as f32 - 32.);
            println! {"       <td>{}°F</td>\n       <td>{:.1}°C</td>", m, b}
            println!("   </tr>");
            m += z;
        }
        println!("</table>")
    }
    if x > y {
        // ex. 300..0
        let mut n = x;
        while n >= y {
            println!("   <tr>");
            let c: f32 = (5. / 9.) * (n as f32 - 32.);
            println! {"       <td>{}°F</td>\n       <td>{:.1}°C</td>", n, c}
            println!("   </tr>");
            n -= z;
        }
        println!("</table>")
    }
}
