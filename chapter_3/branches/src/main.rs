fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else if number == 100 {
        println!("lucky number hit!");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // must be same type

    println!("The value of number is: {number}");
}
