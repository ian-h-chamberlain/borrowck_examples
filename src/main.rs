/// Examples for borrow checking in Rust

fn main() {
    example_1();
    example_2();
    example_3();
    example_4();
}

fn example_1() {
    let mut foo = 3;

    let bar = &mut foo;

    // Not allowed because `bar` is already borrowing `foo`
    let baz = &mut foo;

    *baz = 47;

    *bar += 1;

    println!("Example 1: {}", foo);
}

fn example_2() {
    let mut my_string = String::from("hello");

    let ref1 = &mut my_string;

    // not allowed because ref1 is borrowing `my_string` mutably
    let ref2 = &my_string;

    *ref1 = String::from("foo");

    println!("Example 2: {}, {}", my_string, *ref2);
}

fn example_3() {
    let mut my_string = String::from("hello");

    let ref1 = &my_string;
    let ref2 = &my_string;

    // Not allowed because `ref1` and `ref2` already borrow `my_string` immutably
    let ref3 = &mut my_string;

    *ref3 = String::from("foo");

    println!("Example 3: {}, {}", *ref1, *ref2);
}

fn example_4() {
    let ref1: &mut String;
    {
        let mut my_string = String::from("hello");

        // not allowed because `my_string` goes out of scope
        ref1 = &mut my_string;
    }

    *ref1 = String::from("my fair lady");

    println!("Example 4: {}", *ref1);
}
