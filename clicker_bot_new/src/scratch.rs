use enigo::*;
use std::{thread, time};
use rand::Rng;


fn hypotenuse(a: f32, b: f32) -> f32{
    // Returns the hypothenuse of a triangle.
    //
    // a^2 + b2^ = c^2
    //
    // In this application, and b will always be screen coordinates
    //   which are integers.

    let a = f32::powi(a, 2);
    let b = f32::powi(b, 2);
    let c: f32 = (a + b).sqrt();

}

fn wind_mouse(
    start_x: i32,
    start_y: i32,
    destination_x: i32,
    destination_y: i32,
    destination_precision: i32,
    wind: &mut f32,
    gravity: f32,
    wind_change_distance: f32,
    step_size: & mut f32,
    step_wait: u64,
){
    // WindMouse algorithm. Calls the move_mouse kwarg with each new step.
    // Author: Benjamin J. Land
    // Source: https://ben.land/post/2021/04/25/windmouse-human-mouse-movement/

    // Accessory variables for randomized change
    let sqrt3 = f32::sqrt(3 as f32);
    let sqrt5 = f32::sqrt(3 as f32);
    let mut rng = rand::thread_rng();
    let step_wait = time::Duration::from_millis(step_wait);

    // Cast our integer mouse coordinates to floats
    let destination_x = destination_x as f32;
    let destination_y = destination_y as f32;
    let destination_precision = destination_precision as f32;
    let start_x = start_x as f32;
    let start_y = start_y as f32;
    let mut current_x = start_x.clone();
    let mut current_y = start_y.clone();
    //let mut move_x;
    //let mut move_y;

    // ------------------------------------------------------------------------
    // Force variables

    // let mut wind_x = 0.0;
    // let mut wind_y = 0.0;
    let mut velocity_x = 0.0;
    let mut velocity_y = 0.0;

    let mut relative_gravity: f32;

    // let mut wind_random_x;
    // let mut wind_random_y;
    // let mut velocity;
    // let mut velocity_clip;

    // ------------------------------------------------------------------------
    // Distance variables

    // Store the total distance we have to travel, will use later to set velocity
    // based on the relative proximity to our destination x and y coordinates.
    let distance_total_x = destination_x - start_x;
    let distance_total_y = destination_y - start_y;
    let distance_total = hypotenuse(distance_total_x, distance_total_y);
    // Current distance remaining to travel, controls while loop
    let mut distance = distance_total.clone();

    // ------------------------------------------------------------------------
    // Simple Movement

    let mut enigo = Enigo::new();

    while distance >= destination_precision {

        // The closer we get, the strong gravity becomes
        relative_gravity = gravity / (distance/distance_total);
        velocity_x = gravity;
        velocity_y = gravity;
        current_x += velocity_x;
        current_y += velocity_y;
        thread::sleep(step_wait);
        enigo.mouse_move_to(current_x as i32, current_y as i32);

        distance = hypotenuse(destination_x - current_x, destination_y - current_y);

        println!("start:        {}, {}", start_x, start_y);
        println!("destination:  {}, {}", destination_x, destination_y);
        println!("current:      {}, {}", current_x, current_y);
        println!("distance:     {}", distance);
        println!("velocity:     {}, {}", velocity_x, velocity_y,);
        //println!("wind:         {}, {}, {}", *wind, wind_x, wind_y);
        println!("gravity:      {}", gravity);
        println!("relative_gravity:      {}", relative_gravity);
        // println!("step_size:    {}",step_size);
    }

    // // The goal is to land us within X pixels of our destination.
    // while distance >= destination_precision {

    //     println!("------------------------------------");
    //     // Set the wind to what ever is smaller, current wind or distance remaining.
    //     *wind = f32::min(*wind, distance);

    //     // If we are far enough away from our destination, create random wind change
    //     if distance >= wind_change_distance {
    //         //wind_random_x = (2.0 * rng.gen_range(0.0..1.0) - 1.0) * (*wind / sqrt5);
    //         //wind_random_y = (2.0 * rng.gen_range(0.0..1.0) - 1.0) * (*wind / sqrt5);
    //         wind_random_x = rng.gen_range(0.0-*wind..*wind);
    //         wind_random_y = rng.gen_range(0.0-*wind..*wind);
    //         // wind_x = (wind_x / sqrt3) + wind_random_x;
    //         // wind_y = (wind_y / sqrt5) + wind_random_y;
    //         wind_x = wind_random_x;
    //         wind_y = wind_random_y;
    //     }
    //     // If we are too close to our destination, create small constant wind change
    //     // Also adjust the distance we travel in each step (step size)
    //     else {
    //         wind_x /= sqrt3;
    //         wind_y /= sqrt3;
    //         // If our step size is still large, shrink it a little bit
    //         if *step_size >= 3.0 {
    //             *step_size /= sqrt5;
    //         }
    //         // If our step size is small, start randomizing it
    //         else {
    //             *step_size = rng.gen_range(0.0..1.0) * 3.0 + 3.0;
    //         }
    //     }

    //     // Adjust velocity based on proportional distance to destination
    //     velocity_x += wind_x + gravity; //* (distance_total_x as f32) / distance;
    //     velocity_y += wind_y + gravity; //* (distance_total_y as f32) / distance;
    //     velocity = hypotenuse(velocity_x, velocity_y);

    //     velocity_clip = *step_size / 2.0 + rng.gen_range(0.0..1.0) * *step_size/2.0;
    //     // // If we are moving faster than our step size (overtraveling)
    //     // if velocity > *step_size {
    //     //     velocity_clip = *step_size / 2.0 + rng.gen_range(0.0..1.0) * *step_size/2.0;
    //     //     velocity_x = (velocity_x/velocity) * velocity_clip;
    //     //     velocity_y = (velocity_y/velocity) * velocity_clip;
    //     // }

    //     // Determine the next coordinates to move to
    //     move_x = current_x + velocity_x;
    //     move_y = current_y + velocity_y;

    //     if current_x != move_x || current_y != move_y{
    //         current_x = move_x;
    //         current_y = move_y;
    //         thread::sleep(step_wait);
    //         enigo.mouse_move_to(current_x as i32, current_y as i32);
    //     }
    //     distance = hypotenuse(destination_x - current_x, destination_y - current_y);

    //     // println!("start:        {}, {}", start_x, start_y);
    //     // println!("destination:  {}, {}", destination_x, destination_y);
    //     println!("current:      {}, {}", current_x, current_y);
    //     // println!("distance:     {}", distance);
    //     println!("velocity:     {}, {}", velocity_x, velocity_y,);
    //     println!("wind:         {}, {}, {}", *wind, wind_x, wind_y);
    //     println!("gravity:      {}", gravity);
    //     // println!("step_size:    {}",step_size);

    // }
}

fn main(){

    let mut wind = 100.0; // pixels per step (range of -/+)
    let gravity = 100.0; // pixels per step
    let wind_change_distance = 12.0;
    let mut step_size = 15.0; // pixels per step
    let start_x = 500;
    let start_y = 500;
    let destination_x = 1000;
    let destination_y = 1000;
    let destination_precision = 10;
    let step_wait = 1000;

    let mut enigo = Enigo::new();

    let wait = time::Duration::from_millis(2000);
    thread::sleep(wait);
    enigo.mouse_move_to(start_x, start_y);
    //enigo.mouse_down(MouseButton::Left);

    wind_mouse(
        start_x,
        start_y,
        destination_x,
        destination_y,
        destination_precision,
        &mut wind,
        gravity,
        wind_change_distance,
        &mut step_size,
        step_wait,
    );
    //enigo.mouse_up(MouseButton::Left);

}
