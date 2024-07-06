use std::mem;

// From https://doc.rust-lang.org/rust-by-example/std/box.html

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    fn boxed_origin() -> Box<Self> {
        // Allocate this point on the heap, and return a pointer to it
        Box::new(Point::origin())
    }
}

// A Rectangle can be specified by where its top left and bottom right
// corners are in space
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // (all the type annotations are superfluous)
    // Stack allocated variables
    let point: Point = Point::origin();
    let rectangle: Rectangle = Rectangle {
        top_left: Point::origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    // Heap allocated rectangle
    //
    //      Stack           (Addr)
    //  |-----------------|
    //  | ...             |
    //  |-----------------|
    //  | boxed_rectangle | (0x0c12)
    //  |  0x1234         | (8 bytes stored here for pointer)
    //  |-----------------|
    //
    //      Heap           (Addr)
    //  |-----------------|
    //  | (0,0)           | (0x1234)
    //  | (3.0, -4.0)     | (32 bytes in heap (16 bytes * 2 points))
    //  |-----------------|
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: Point::origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // The output of functions can be boxed
    let boxed_point: Box<Point> = Box::new(Point::origin());

    // Double indirection
    let box_in_a_box: Box<Box<Point>> = Box::new(Point::boxed_origin());

    println!(
        "Point occupies {} bytes on the stack",
        mem::size_of_val(&point)
    );
    println!(
        "Rectangle occupies {} bytes on the stack",
        mem::size_of_val(&rectangle)
    );

    // box size == pointer size
    println!(
        "Boxed point occupies {} bytes on the stack",
        mem::size_of_val(&boxed_point)
    );
    println!(
        "Boxed rectangle occupies {} bytes on the stack",
        mem::size_of_val(&boxed_rectangle)
    );
    println!(
        "Boxed box occupies {} bytes on the stack",
        mem::size_of_val(&box_in_a_box)
    );

    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!(
        "Unboxed point occupies {} bytes on the stack",
        mem::size_of_val(&unboxed_point)
    );
}
