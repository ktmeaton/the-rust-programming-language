pub mod data_types;
pub mod functions;
pub mod control_flow;

fn main() {

    let sep = "-";

    // Chapter 3.2 Data Types
    let section = "Chapter 3.2 Data Types";
    println!("{}", sep.repeat(80));
    println!("{section}");
    data_types::data_types();

    // Chapter 3.3 Functions
    let section = "Chapter 3.3 Functions";
    println!("{}", sep.repeat(80));
    println!("{section}");
    functions::another_function(3);
    functions::labeled_measurement(5, 'm');
    functions::expressions();
    let x = functions::five();
    print!("The value of x is: {x}\n");

    let x = functions::plus_one(5);
    print!("The value of x is: {x}\n");

    // Chapter 3.5 Control Flow
    let section = "Chapter 3.5 Control Flow";
    println!("{}", sep.repeat(80));
    println!("{section}");

    control_flow::if_1();
    control_flow::if_2(4);
    control_flow::if_2(5);
    control_flow::if_3(true);
    control_flow::if_3(false);
    control_flow::loop_1(10);
    control_flow::loop_2();

    control_flow::while_1();
}
