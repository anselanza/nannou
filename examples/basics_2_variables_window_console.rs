extern crate nannou;

use nannou::prelude::*;

fn main() {
    nannou::run(model, event, view);
}

struct Model {
    window: WindowId,
}

fn model(app: &App) -> Model {
    // Construct and define the size of our window using .with_dimensions(.,.)
    // Argument 1 = width of window; Argument 2 = height of window
    let window = app.new_window().with_dimensions(640,480).build().unwrap();
    
    // Below are the different variable types available in Rust
    let i = 50; // Ints store whole numbers
    let f = 36.6; // Floats are used to store numbers with decimals or fractions of numbers
    let b = true; // Boolean values can be either 'true' or 'false'
    let c = '!'; // Char can only hold a single character
    let message = "hello world"; // Strings hold a collection of characters

    // Print the values stored in our varibales to the console
    println!("i = {}", i);
    println!("f = {}", f);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("message = {}", message);

    Model { window }
}

fn event(_app: &App, model: Model, event: Event) -> Model {
    if let Event::Update(_dt) = event {
    } 
    model
}

fn view(_app: &App, model: &Model, frame: Frame) -> Frame {
    // Our app only has one window, so retrieve this part of the `Frame`. Color it gray.
    frame.window(model.window).unwrap().clear_color(0.1, 0.11, 0.12, 1.0);
    // Return the drawn frame.
    frame
}
