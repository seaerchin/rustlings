// variables3.rs
// Make me compile! Execute the command `rustlings hint variables3` if you want a hint :)

fn main() {
    let x = 3;
    println!("Number {}", x);
    // shadowing occurs here
    // we are reusing the same label but variable data is different.
    let x = 5; // don't change this line
    println!("Number {}", x);
}
