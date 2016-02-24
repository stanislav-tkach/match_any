#[macro_use] extern crate match_any;

use std::any::Any;

fn make_any<T: Any>(value: T) -> Box<Any> {
    Box::new(value)
}

#[test]
fn match_i32() {
    let mut i32_matched = false;

    match_any!(make_any(10) => 
        _x: i8 => { 
            assert!(false);
        }
        _x: i16 => { 
            assert!(false);
        }
        _x: i64 => { 
            assert!(false);
        }
        x: i32 => { 
            assert_eq!(*x, 10);
            i32_matched = true;
        }
    );

    assert!(i32_matched);
}

// TODO: Add #[bench].
