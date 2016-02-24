#[macro_use] extern crate match_any;

#[test]
fn empty() {
    let a: Box<std::any::Any> = Box::new(10);
    println!("{:?}", a);
    
    match_any!(a => 
        x: i32 => { 
            assert_eq!(*x, 10);
        }
    );
}

// TODO: Add.
// #[bench]
