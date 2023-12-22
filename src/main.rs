mod day01;

fn main() {
    let now = std::time::Instant::now();
    day01::s1();
    let ts1 = now.elapsed().as_secs_f32();

    let now = std::time::Instant::now();
    day01::s2();
    let ts2 = now.elapsed().as_secs_f32();
    println!("ts1={ts1}\nts2={ts2}");
    
}
