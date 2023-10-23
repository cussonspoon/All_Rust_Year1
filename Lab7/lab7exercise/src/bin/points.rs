use rand::Rng;

fn gen_number<R: Rng>(rng: &mut R) -> (f64, f64) {
    let num: f64 = rng.gen_range(-10.0..=10.0);
    let num2: f64 = rng.gen_range(-10.0..=10.0);
    (num, num2)
}
fn main(){
    let arg: Vec<String> = std::env::args().collect();
    let n_arg = if arg.len() < 2 {""} else {&arg[1]};
    let n : i64 = n_arg.parse().expect("No argument provided");
    let mut rng = rand::thread_rng();
    let mut count = 0.0;
    
    for _ in 0..n{
        let num : (f64, f64) = gen_number(&mut rng );
        let (x, y) = num;
        if (x.powf(2.0)+y.powf(2.0)).sqrt() <= 1.{
            count +=1.
        }
    }
    let p = count*100./n as f64;
    println!("{}%", p*100.);
    print!("P*4 = {}", p*4.)
}