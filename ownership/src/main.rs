fn main() {
    let mut s = String::from("Hello");

    s.push_str(" World");

    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    let mut x = 20;
    let y = x;
    x = 21;

    println!("{}, world!, {}, {}, {}", s1, s2, x, y);

    let str = String::from("Hello");

    takes_ownership(str);

    let str1 = gives_ownership();

    let str2 = String::from("hello");
    let str3 = takes_and_gives_back(str2);

    let (s2, len) = calculate_length(str1);
    let len2 = calculate_length_by_ref(&str3);

    println!("String is {}, {}, {}, {}", s2, str3, len, len2);
    println!("Reference: {}", len2);

    let mut s = String::from("String");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

fn calculate_length_by_ref(s: &String) -> usize {
    s.len()
}
