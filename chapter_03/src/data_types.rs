pub fn data_types() {
    // ------------------------------------------------------------------------
    // Scalar Data
    // ------------------------------------------------------------------------

    // Integers
    // Unsigned: 0 to 2^8 - 1
    // Signed: -(2^(n-1)) to 2^(n-1) - 1
    let int_u_8_bit: u8 = 1;
    println!("The 8 bit data is: {int_u_8_bit}");

    // If we allow overflow, 256 loops back around to 0
    #[allow(overflowing_literals)]
    let int_u_8_bit_bad: u8 = 256;
    println!("The 8 bit data is: {int_u_8_bit_bad}");

    // Decimal: 98_222 = 98222
    let int_dec: i32 = 98_222;
    println!("The decimal data is: {int_dec}");
    // Hex: 0xff = 255
    let int_hex = 0xff;
    println!("The hex data is: {int_hex}");

    // ------------------------------------------------------------------------
    // Compound Data
    // ------------------------------------------------------------------------

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Underscores to mark unused var
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    // I think  this shadows
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    // Arrays
    // Simple
    let a = [1, 2, 3, 4, 5];
    // Strict
    //let b: [i32; 5] = [1, 2, 3, 4, 5];
    // Repeat
    //let c = [3; 5];

    let first = a[0];
    //let second = a[1];

    println!("The value of first is: {first}");

    // Parse array index from user input
    println!("Please enter an array index:");
    // let mut index = String::new();
    // std::io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line.");
    let index = "1";

    // Cast from string to unsigned inteter
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");

    let element = a[index];

    println!("The value of index {index} is {element}.");
}
