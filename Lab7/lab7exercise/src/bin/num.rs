use rand::Rng;

fn gen_number<R: Rng>(rng: &mut R) -> f64 {
    let num: f64 = rng.gen_range(-10.0..=10.0);
    num
}
fn main(){
    let arg: Vec<String> = std::env::args().collect();
    let n_arg = if arg.len() < 2 {""} else {&arg[1]};
    let n : i64 = n_arg.parse().expect("No argument provided");
    let mut rng = rand::thread_rng();
    let mut count = 0.0;
    
    for _ in 0..n{
        let num : f64 = gen_number(&mut rng );
        if num >= -1. && num <= 1.{
            count +=1.;
        }
    }
    let p = (count/n as f64)*100.;
    print!("{}%", p)
}

