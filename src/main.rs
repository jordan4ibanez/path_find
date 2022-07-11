use rand::{thread_rng, Rng, prelude::ThreadRng};

fn main() {

    let mut randy: ThreadRng = thread_rng();

    

    for i in 0..1000 {

        let test: i8 = randy.gen_range(0..127);

        println!("YOUR NUMBA! {}", i + i32::from(test));

        // drop(test);
    }

    println!("Hello, world!");

}
