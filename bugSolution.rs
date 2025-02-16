use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    let y = x.borrow_mut();
    *y = 6;
    // Note that you can only have one mutable borrow at a time.  
    //Trying to borrow again with z will cause a runtime panic
    //let z = x.borrow_mut();
    // *z = 7;
    println!("x = {}", x.borrow());
}
