fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    // compound types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("{x}");
    println!("{y}");
    println!("{z}");

    let tup = ('😂', '😁', '🙂');
    let laugh = tup.0;
    println!("{laugh}");

    let a: [i32; 3] = [1, 2, 3];
    let a = [3; 5];
    let element = a[5];
    println!("{element}");

}
