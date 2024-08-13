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
}
