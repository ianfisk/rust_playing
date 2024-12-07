mod graph {
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct Node<T> {
        pub value: T,
        pub children: Vec<Rc<Node<T>>>,
    }

    impl<T> Node<T> {
        pub fn new(value: T) -> Node<T> {
            Node {
                value,
                children: vec![],
            }
        }

        pub fn new_with_child(value: T, child: Rc<Node<T>>) -> Node<T> {
            Node {
                value,
                children: vec![child],
            }
        }

        pub fn new_with_children(value: T, children: Vec<Rc<Node<T>>>) -> Node<T> {
            // children should be wholly owned by the Node. Caller must clone the vec if necessary.
            Node { value, children }
        }
    }
}

use graph::Node;
use std::rc::Rc;

fn make_graph() -> Vec<Node<i32>> {
    let a = Rc::new(Node::new(0));
    let b = Rc::new(Node::new_with_child(1, Rc::clone(&a)));
    let c = Rc::new(Node::new_with_child(2, Rc::clone(&a)));

    let d: Node<i32>;
    let e: Node<i32>;
    {
        // Playing with reference count here. Wrapping this block in its own scope means the
        // incremented reference counts of b and c from the children variable don't persist
        // after we create d and e.
        let children = vec![Rc::clone(&b), Rc::clone(&c)];
        d = Node::new_with_children(3, children.clone());
        e = Node::new_with_children(4, children.clone());
    }

    println!("root b {:?}", b);
    println!("root c {:?}", c);
    println!("root d {:?}", d);
    println!("root e {:?}", e);

    println!("strong count for a {:?}", Rc::strong_count(&a));
    println!("strong count for b {:?}", Rc::strong_count(&b));
    println!("strong count for c {:?}", Rc::strong_count(&c));

    return vec![d, e];
}

fn main() {
    let graph_roots = make_graph();

    // IMPORTANT: Here we need to iterate through the graph and children using a reference or else
    // the loop takes ownership and effects the reference count(!!!)
    for node in &graph_roots {
        for child in &node.children {
            println!(
                "Node {}: strong count for child {:?}: {:?}",
                node.value,
                child,
                Rc::strong_count(&child)
            );
        }
    }

    println!("{:?}", graph_roots);
}
