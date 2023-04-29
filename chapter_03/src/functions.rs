pub fn another_function(x: i32){
    println!("The value of x is: {x}");
}

pub fn labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

pub fn expressions(){
    let y = {
        let x = 3;
        println!("The value of x is: {x}");
        x + 1
    };
    println!("The value of y is: {y}");
}

pub fn five() -> i32 {
    5
}

pub fn plus_one(x: i32) -> i32 {
    x + 1
}
