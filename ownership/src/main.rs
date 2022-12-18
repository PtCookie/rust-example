//fn main() {
//    let s1 = String::from("hello");
//    let s2 = s1.clone();
//
//    println!("s1 = {}, s2 = {}", s1, s2);
//
//    let x = 5;
//    let y = x;
//
//    println!("x = {}, y = {}", x, y);
//}

//fn main() {
//    let s = String::from("hello");
//
//    takes_ownership(s);
//
//    let x = 5;
//
//    makes_copy(x);
//    let y = x;
//
//    println!("y: {}", y);
//}
//
//fn takes_ownership(some_string: String) {
//    println!("{}", some_string);
//}
//
//fn makes_copy(some_integer: i32) {
//    println!("{}", some_integer);
//}

//fn main() {
//    let s1 = String::from("hello");
//
//    let len = calculate_length(&s1);
//
//    println!("The length of '{}' is {}.", s1, len);
//}
//
//fn calculate_length(s: &String) -> usize {
//    s.len()
//}

//fn main() {
//    let mut s = String::from("hello");
//
//    let r = &s;
//    println!("{}", r);
//
//    change(&mut s);
//
//    let r = &s;
//    println!("{}", r);
//}
//
//fn change(some_string: &mut String) {
//    some_string.push_str(", world");
//}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
