mod graph {
    use std::{cell::RefCell, rc::Rc};

    // Is it "rust-y" to alias a type like this so it's easier to work with?
    pub type NodeRef<T> = Rc<RefCell<Node<T>>>;

    #[derive(Debug, PartialEq)]
    pub struct Node<T: PartialEq> {
        pub value: T,
        pub children: Vec<NodeRef<T>>,
    }

    impl<T: PartialEq> Node<T> {
        pub fn new(value: T) -> Node<T> {
            Node {
                value,
                children: vec![],
            }
        }

        pub fn new_with_child(value: T, child: NodeRef<T>) -> Node<T> {
            Node {
                value,
                children: vec![child],
            }
        }

        pub fn new_with_children(value: T, children: Vec<NodeRef<T>>) -> Node<T> {
            // children should be wholly owned by the Node. Caller must clone the vec if necessary.
            Node { value, children }
        }

        pub fn add_child(&mut self, child: NodeRef<T>) {
            self.children.push(child);
        }

        pub fn has_descendant(&self, node: &NodeRef<T>) -> bool {
            if self.children.contains(node) {
                return true;
            }

            return self
                .children
                .iter()
                .find(|child| child.borrow().has_descendant(node))
                .is_some();
        }
    }
}

use graph::Node;
use std::{cell::RefCell, rc::Rc};

fn make_graph() -> Vec<Node<i32>> {
    let a = Rc::new(RefCell::new(Node::new(0)));
    let b = Rc::new(RefCell::new(Node::new_with_child(1, Rc::clone(&a))));
    let c = Rc::new(RefCell::new(Node::new_with_child(2, Rc::clone(&a))));

    let d: Node<i32>;
    let e: Node<i32>;
    let f: Node<i32>;
    {
        // Playing with reference count here. Wrapping this block in its own scope means the
        // incremented reference counts of b and c from the children variable don't persist
        // after we create d and e, if d and e only live in this scope.
        // let d: Node<i32>;
        // let e: Node<i32>;
        let children = vec![Rc::clone(&b), Rc::clone(&c)];
        d = Node::new_with_children(3, children.clone());
        e = Node::new_with_children(4, children.clone());
        f = Node::new_with_child(5, Rc::clone(&b));
    }

    println!("root b {:?}", b);
    println!("root c {:?}", c);
    println!("root d {:?}", d);
    println!("root e {:?}", e);
    println!("root f {:?}", f);

    println!(
        "strong count for a {:?} (should be 3)",
        Rc::strong_count(&a)
    );
    println!(
        "strong count for b {:?} (should be 4)",
        Rc::strong_count(&b)
    );
    println!(
        "strong count for c {:?} (should be 3)",
        Rc::strong_count(&c)
    );

    return vec![d, e, f];
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
