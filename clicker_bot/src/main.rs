use enigo::*;
use std::{thread, time};

struct MouseCoord {
    x : i32,
    y : i32,
}

fn main(){
    let mut enigo = Enigo::new();

    //let (x, y) = enigo.mouse_location();
    let x = 500;
    let y = 500;
    let wait = time::Duration::from_millis(500);

    let origin = MouseCoord { x:x, y:y };
    let destination = MouseCoord { x:1000, y:1000 };
    let mut current = MouseCoord { x:origin.x, y:origin.y};

    println!("Origin: {}, {}", origin.x, origin.y);
    enigo.mouse_move_to(origin.x, origin.y);

    let x_dist = destination.x - origin.x;
    let y_dist = destination.y - origin.y;

    println!("Destination: {}, {}", destination.x, destination.y);
    println!("Distance: {}, {}", x_dist, y_dist);

    let mut step = 0;
    let steps = 10;

    let x_inc = x_dist / steps;
    let y_inc = y_dist / steps;

    while step < steps {
        thread::sleep(wait);

        current.x = current.x + x_inc;
        current.y = current.y + y_inc;
        step += 1;
        println!("Step {step}: {x_inc}, {y_inc}");
        enigo.mouse_move_to(current.x, current.y);
    }


    // What is the halfway point
    //thread::sleep(wait);
    //enigo.mouse_move_to(destination.x, destination.y);

}
