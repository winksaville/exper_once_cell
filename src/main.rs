use std::thread;

use exper_once_cell::{Thing, RANDOM, THING};

fn main() {
    let thing = &THING;
    use_thing(thing);
    use_two_threads();
}

fn use_thing(thing: &Thing) {
    println!("vec: {:?}", thing.vec);

    let rs = &RANDOM.to_string();
    let thing = &THING;

    let first = thing.vec.first().unwrap();
    println!("use_thing: first={first}");
    let s = format!("one {rs}");
    assert_eq!(s, first.as_str());

    let last = thing.vec.last().unwrap();
    println!("use_thing: last={first}");
    let s = format!("two {rs}");
    assert_eq!(s, last.as_str());
}

fn use_two_threads() {
    // Get the expected values in the main thread
    let rs = &RANDOM.to_string();
    let fi: String = format!("one {rs}");
    let si: String = format!("two {rs}");

    // In the scoped thread verify they are the expected values
    thread::scope(|s| {
        s.spawn(|| {
            let thing = &THING;

            let first = thing.vec.first().unwrap();
            println!("use_two_threads thread: first={first}");
            assert_eq!(fi, first.as_str());

            let last = thing.vec.last().unwrap();
            println!("use_two_threads thread: last={last}");
            assert_eq!(si, last.as_str());
        });
    });

    // Now in the main thread verify they are the expected values
    let thing = &THING;

    let first = thing.vec.first().unwrap();
    println!("use_two_threads main: first={first}");
    assert_eq!(fi, first.as_str());

    let last = thing.vec.last().unwrap();
    println!("use_two_threads main: last={last}");
    assert_eq!(si, last.as_str());
}
