use std::io;

fn calc_mars_weight(weight: f32) -> f32{
    (weight / 9.81) * 3.711
}

fn main() {
    println!("Enter your Weight(kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    let weight = input.trim().parse().unwrap();
    let mut mars_weight = calc_mars_weight(weight);
  println!("Your weight on Mars would be {:.2}(kg)", mars_weight);
}
