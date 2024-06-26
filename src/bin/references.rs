fn main() {
    let i = 100;
    let j: &i32 = &i;
    let jj: &&i32 = &j; // Can also be typed as &i32 because Rust has implicit conversions (so-called “coercion”) that can remove extra layers of references.
    let jjj = &jj;
    let k = &&i; // same as jj

    // Where is the value of i in memory? (Location on the stack)
    println!("The value of i is at {:p}", &i);
    print_type_of(&i);
    // Where is the reference variable j on the stack? This should be the next variable on the stack after i => 4 bytes above i in memory.
    println!(
        "&j {:p} (j is at this address. It should be 4 bytes above i on the stack)",
        &j
    );
    // Where is jj on the stack? This should be 8 bytes (sizeof ref on x64 machine) above j on the stack.
    println!(
        "&jj {:p} (jj is at this address. It should be 8 bytes above j)",
        &jj
    );
    println!("&jjj {:p} (...)", &jjj);
    println!("&k {:p} (...)", &k);
    println!();

    println!(
        "The value j references is at {:p} (This should match &i)",
        j
    );
    // Where is jj pointing? It points to j, which is the next variable on the stack after i, so the address jj holds should be addr(i) + 4 bytes.
    println!(
        "(jj = &j) The value jj references is at {:p} (This should be the address of j)",
        jj
    );
    println!(
        "(jjj = &jj) The value jjj references is at {:p} (This should be the address jj)",
        jjj
    );
    println!("k = &&i {:p} (Why is the value k is referencing the next available space on the stack? I.e., the next memory slot above itself)", k);
    println!();

    println!("So k is at... &k = {:p}", &k);
    println!("And this slot of memory on the stack contains the address {:p} (k is pointing to next slot on the stack)", k);
    println!(
        "And this address {:p} references what? {:p}  <--- THIS IS THE ADDRESS OF i",
        k, *k
    );
    println!();

    /*
     *      Stack      (Addr)
     *  |------------|
     *  | i = 100    | (0x04)
     *  |------------|
     *  | j = 0x04   | (0x08) <-- 4 bytes above i because i is i32
     *  |            |   (j is pointing to i)
     *  |------------|
     *  | jj = 0x08  | (0x10) <-- 8 bytes above j because the size of a reference is 8 bytes on a x64 system
     *  |            |   (jj is pointing to j)
     *  |------------|
     *  | jjj = 0x10 | (0x18)
     *  |            |   (jjj is pointing to jj)
     *  |------------|
     *  | k = 0x28   | (0x20)
     *  |            |   (k is pointing to next stack slot)
     *  |------------|
     *  |   0x04     | (0x28)
     *  |            |   (this stack slot points back to i)
     *  |------------|
     *
     * (Stack grows up on Rust playground)
     */

    // println! dereferences to focus on the underlying value (i = 100);
    // So, using {:p} is the Pointer format outputs the memory address stored in the variable.
    // Reminder: references == type that stores a memory address.
    println!("j = {}", j);
    println!("jj = {}", jj);
    println!("jjj = {}", jjj);
    println!("k = {}", k);
    println!();

    let x: i32 = 1;
    let ptr_y: &i32 = &x;
    println!("x: {}, ptr_y: {}", x, *ptr_y);
    println!("x: {}, ptr_y: {}", x, ptr_y);
    println!("x: {:p}, ptr_y: {:p}", &x, ptr_y);
    print_type_of(&x);
    print_type_of(ptr_y);
}

fn print_type_of<T>(x: &T) {
    println!("{} at {:p}", std::any::type_name::<T>(), x);
}
