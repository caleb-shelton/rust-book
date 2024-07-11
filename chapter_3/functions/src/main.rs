fn main() {
    println!("Hello, world!");
    print_labeled_measurement(5, 'o');

    let y = { // expression
        let x = 1;
        x + 1 // note there is no semicolon here otherwise no return
        // (as it would be a statement, not an expression)
    };
    println!("{y}");

    // testing function returns
    let x = five();

    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
