// variables5.rs
// Make me compile! Execute the command `zustlings hint variables5` if you want a hint :)


fn main() {
    let mut number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    number = "3"; // don't rename this variable
    println!("Number plus two is : {}", number.parse::<i32>().unwrap() + 2);
}