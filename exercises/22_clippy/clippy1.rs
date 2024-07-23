fn main() {
    let pi = std::f32::consts::PI; // Use the precise value of π for f32
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}
