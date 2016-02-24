#[macro_use] extern crate match_any;

use std::any::Any;

fn make_any<T: Any>(value: T) -> Box<Any> {
    Box::new(value)
}

#[test]
fn match_i32() {
    match_any!(make_any(10) => 
        x: i32 => { 
            assert_eq!(*x, 10);
        }
    );
}

// TODO: Add #[bench].
