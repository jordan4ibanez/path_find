use rand::{thread_rng, Rng, prelude::ThreadRng};

fn main() {

    const ARRAY_LEN: usize = 1000;

    let mut randy: ThreadRng = thread_rng();

    let mut my_array: [i32; ARRAY_LEN] = [0; ARRAY_LEN];
    

    for i in 0..ARRAY_LEN {

        let test: i32 = randy.gen_range(0..100);

        my_array[i] = test;
    }

    for _ in 0..ARRAY_LEN {
        let test_2 = my_array[randy.gen_range(0..ARRAY_LEN)];

        println!("YOUR SPECIAL NUMBER IS! {}", test_2);
    }



    println!("Hello, world!");

}
