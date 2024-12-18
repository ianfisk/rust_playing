#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared_node = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&shared_node), Rc::new(Nil)));
    let aa = Rc::new(Cons(Rc::clone(&shared_node), Rc::clone(&a)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&aa));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&aa));

    *shared_node.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("aa after = {aa:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

    // a is referenced once by its variable and once by aa.
    // aa is reference by its variable, b, and c.
    println!("Reference count of a = {}", Rc::strong_count(&a));
    println!("Reference count of aa = {}", Rc::strong_count(&aa));

    // shared_node is referenced once by its variable, a, and aa.
    println!(
        "Reference count of shared_node = {}",
        Rc::strong_count(&shared_node)
    );

    // This should remove 2 references to aa. New total = 1;
    drop(b);
    drop(c);
    println!(
        "Reference count of aa after dropping b and c = {}",
        Rc::strong_count(&aa)
    );

    // Dropping `a` here doesn't affect the reference count of shared_node
    // because the list that the variable `a` pointed to is still referenced
    // by aa (a's ref count = 1 now) and therefore it's not dropped yet. Since
    // `a` still ~exists, shared_node is still referenced by its own variable,
    // the list that `a` used to point to, and the list that aa points to
    // (still 3).
    drop(a);
    println!(
        "Reference count of shared_node remains unchanged after dropping a = {}",
        Rc::strong_count(&shared_node)
    );

    // Dropping aa will removed two references to the shared node: one from
    // aa itself and one from the ghost of `a`. The ghost of a is now cleaned
    // up because its reference count moves to 0 when aa is dropped.
    drop(aa);
    println!(
        "Reference count of shared_node drops by 2 after dropping aa = {}",
        Rc::strong_count(&shared_node)
    );
}
