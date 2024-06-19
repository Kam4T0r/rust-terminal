use rand::Rng;

pub fn matrix(){
    loop {
        let rng_number = rand::thread_rng().gen_range(1..i32::MAX);
        println!("{}{}{}{}{}{}",rng_number,rng_number,rng_number,rng_number,rng_number,rng_number);
    }
}