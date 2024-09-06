fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str appends a literal to a String

    println!("{s}");

    // error showcase and fix with clone()
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{s1}, world!");
}
