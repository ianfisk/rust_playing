use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // Next line incorrectly tries to borrow the moved value.
    // println!("Before defining closure: {list:?}");
}
