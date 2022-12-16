use once_cell::sync::Lazy;
use rand::prelude::*;

// Singleton Random number so we can verify only ran once
pub static RANDOM: Lazy<u32> = Lazy::new(random);

// A Thing with a vector which will be initialized once
pub static THING: Lazy<Thing> = Lazy::new(Thing::new);

#[derive(Debug)]
pub struct Thing {
    pub vec: Vec<String>,
}

impl Thing {
    fn new() -> Self {
        let mut cfg = Thing { vec: Vec::new() };

        let r = &RANDOM;
        let rs = r.to_string();
        let s = format!("one {rs}");
        println!("s={s}");
        cfg.vec.push(s);

        let s = format!("two {rs}");
        println!("s={s}");
        cfg.vec.push(s);

        cfg
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn test_single_thread() {
        // Get the random number as a string
        let rs = &RANDOM.to_string();

        // Get a reference to thing
        let thing = &THING;

        // Verify the first item
        let first = thing.vec.first().unwrap();
        let s = format!("one {rs}");
        assert_eq!(s, first.as_str());

        // Verify the last item
        let last = thing.vec.last().unwrap();
        let s = format!("two {rs}");
        assert_eq!(s, last.as_str());
    }

    #[test]
    fn test_two_threads() {
        // Get the expected values in the main thread
        let rs = &RANDOM.to_string();
        let fi: String = format!("one {rs}");
        let si: String = format!("two {rs}");

        // In the scoped thread verify they are the expected values
        thread::scope(|s| {
            s.spawn(|| {
                // Get a reference to thing
                let thing = &THING;

                // Verify the first item
                let first = thing.vec.first().unwrap();
                assert_eq!(fi, first.as_str());

                // Verify the last item
                let last = thing.vec.last().unwrap();
                assert_eq!(si, last.as_str());
            });
        });

        // Now in the main thread verify they are the expected values

        // Get a reference to thing
        let thing = &THING;

        // Verify the first item
        let first = thing.vec.first().unwrap();
        assert_eq!(fi, first.as_str());

        // Verify the last item
        let last = thing.vec.last().unwrap();
        assert_eq!(si, last.as_str());
    }
}
