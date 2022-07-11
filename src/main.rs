
use std::fmt::Debug;

use notan::prelude::*;

use notan::draw::*;

use rand::{thread_rng, Rng, prelude::ThreadRng};

#[derive(AppState)]
struct State {
    last_key: Option<KeyCode>,
}


fn setup() -> State {
    
    State {
        last_key: None,
    }     

}


fn update(app: &mut App, state: &mut State) {
    
    state.last_key = app.keyboard.last_key_released();

    if app.keyboard.is_down(KeyCode::Escape) {
        app.exit();
        println!("quiting");
        return;
    }


    const ARRAY_LEN: usize = 1000;

    let mut randy: ThreadRng = thread_rng();

    let mut my_array: [i32; ARRAY_LEN] = [0; ARRAY_LEN];
    

    for i in 0..ARRAY_LEN {

        let test: i32 = randy.gen_range(0..100);

        my_array[i] = test;
    }

    for _ in 0..ARRAY_LEN {
        let test_2 = my_array[randy.gen_range(0..ARRAY_LEN)];

        // println!("YOUR SPECIAL NUMBER IS! {}", test_2);
    }

    println!("Hello, world!");

}

#[notan_main]
fn main() -> Result<(), String> {

    // test_thing();

    notan::init_with(setup)
        .update(update)
        .build()

}
