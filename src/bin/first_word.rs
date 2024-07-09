#![feature(vec_into_raw_parts)]

fn main() {
    let s = String::from("Hello, world!");
    let word = first_word(&s);
    println!("First word of {}: {}", s, word);
    let word_with_ownership = first_word_gives_ownership(&s);
    println!("Owned first word: {}", word_with_ownership);
    println!("Original string s reference is on stack at {:p}", &s);
    println!(
        "Owned copy of the first word reference on the stack at {:p}",
        &word_with_ownership
    );

    let ptr_s = s.into_raw_parts();
    println!("Original string s heap memory at {:?}", ptr_s);
    let ptr_owned_first_word = word_with_ownership.into_raw_parts();
    println!(
        "Owned copy of the first word heap memory at {:?}",
        ptr_owned_first_word
    );
    println!();

    let t = String::from("Hello,world!");
    let word = first_word(&t);
    println!("First word of {}: {}", t, word);

    let blank = String::new();
    let word = first_word(&blank);
    println!("First word of {}: {}", blank, word);

    // first_word(literal) won't compile if first_word is defined as
    // fn first_word(s: &String) -> &str; because literals are slices.
    let literal = "Literal, I am";
    println!("Literal's first word: {}", first_word(literal));

    // // Comment out the `let ptr_s = s.into_raw_parts();` line and then we can still use s here
    // // because we have not transferred ownership and it is still in scope.
    // println!("s is still valid: {}", s);
    // let p = s; // Ownership move
    // println!("s now moved to p: {}", p);
    // // println!("p {} now has ownership of s {}", p, s); // This line will not compile.
}

// Before lifetime elision this would be: fn first_word<'a>(s: &'a str) -> &'a str { ... }
fn first_word(s: &str) -> &str {
    let mut chars = s.chars();
    let first_space_index: Option<usize> = chars.position(|x| x == ' ');
    match first_space_index {
        Some(i) => &s[..i],
        None => s,
    }
}

fn first_word_gives_ownership(s: &str) -> String {
    String::from_iter(s.chars().take_while(|&x| x != ' '))
}
