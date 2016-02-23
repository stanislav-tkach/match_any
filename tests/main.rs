#[macro_use] extern crate match_any;

#[test]
fn empty() {
    let a: Box<std::any::Any> = Box::new(10);
    println!("{:?}", a);
    
    match_any!(a => 
        x: i32 => { 
            println!("i32 = {}", x);
        }
    );
}
