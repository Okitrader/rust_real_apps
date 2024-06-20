use std::io;

fn main() {
    //println!("Number: {}, String: {}", 100, "abcd");// ! is macro

    println!("Enter your weight in kilograms:  ");
    let mut input = String::new(); // creates an empty string

    io::stdin().read_line(&mut input).unwrap(); // passes in the empty string so that the user can input


    let weight= input.trim().parse().unwrap();
    let earth_weight: f32 = weight;
    let mars_weight = calculate_weight_on_mars(weight);
    let mars_weight_in_grams = mars_weight * 1000.0;
    println!("Your weight on Mars: {} grams or {} kilos, your weight on Earth is {} kg", mars_weight_in_grams, mars_weight, earth_weight);

}
fn calculate_weight_on_mars(weight: f32) -> f32 { // we pass in a f32 and return a f32
    (weight / 9.81) * 3.711 // the last action in a function can return without the return keyword

}

