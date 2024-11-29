fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 3* 60 * 60;

    println!("The value of the constant is {THREE_HOURS_IN_SECONDS}");

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("Value of y in the curly braces scope is {y}");
    }
    println!("Value of y after the curly braces scope is {y}");

    let spaces = "                  ";
    let spaces = spaces.len();
    println!("Values of spaces: {spaces}");
}
