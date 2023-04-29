pub fn if_1(){
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

pub fn if_2(number: i32){

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

pub fn if_3(condition: bool){
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

pub fn loop_1(iter: i32){
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter >= iter {
            break counter * 2;
        }
    };

    println!("The result is: {result}");
}

pub fn loop_2(){
    let mut count = 0;
    println!("\tBeginning loop: counting_up");
    'counting_up: loop {
        println!("\tcount = {count}");
        let mut remaining = 10;

        'counting_down: loop {
            println!("\tremaining = {remaining}");
            if remaining == 9 {
                break 'counting_down;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1
        }

        count += 1;
    }

    println!("End count = {count}");
}

pub fn while_1(){
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    print("LIFTOFF!!!")
}
