fn main() {
    let s = "Hello, world!";

    {
        let s = "goodbye, world!";
        println!("{}", s);
    }

    println!("{}", s);

    let str = String::from("I can't add to this string because it's on the stack");
    println!("{}", str);

    let mut str = String::from("I can add to this string because it's on the heap");
    str.push('!');

    println!("{}", str);

    let s1 = String::from("this is being moved");
    let s2 = s1;
    println!("{}", s2);

    let s3 = String::from("this is being deep copied");
    let s4 = s3.clone();
    println!("{}, {}", s3, s4);

    let x = 5;
    let y = x;
    println!("{} {}, this is not moved, because integers have a fixed size on the stack.", x, y);

    let new_str = String::from("hello");
    takes_ownership(new_str);
    let new_x = 5;
    makes_copy(new_x);
    let str_5 = gives_ownership();
    let str_6 = takes_and_gives_ownership(str_5);
    println!("{}, this was moved and dropped, but has returned a new string to the caller", str_6);
}

fn takes_ownership(some_string: String) {
    println!("{}, you can't use this anymore", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_and_gives_ownership(some_string: String) -> String {
    let some_string = String::from(some_string);
    some_string
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}