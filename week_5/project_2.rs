use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();


println!("How old are you? ");
io::stdin().read_line(&mut input1).expect("Not a valid string");
let a:f32 = input1.trim().parse().expect("Not a valid number");

println!("How many years of work experience do you have? ");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let b:f32 = input2.trim().parse().expect("Not a valid number");
// Assuming the number of years that qualifies as work expereience is 5, and only ages 18 and upwards apply for the job.

if a < 18.00 && b >= a 
{
    println! ("Error: Incorrect information entered.")
}
else if a < 18.00
{
    println! ("You are too young to be an employee.")
}
else if b >= a 
{
    println! ("Error: Incorrect information entered.")
}
else if a >= 40.00 && b >= 5.00
{
    println! ("Your incentive is N1_560_000.000.")
}
else if 30.00 <= a && a < 40.00 && b >=5.00
{
    println! ("Your incentive is N1_480_000.000.")
}
else if a < 28.00 && b >= 5.00
{
    println! ("Your incentive is N1_300_000.000.")
}
else if b < 5.00
{
    println! ("Your incentive is N100_000.000.")
}
else if b > a 
{
    println! ("Error: Incorrect information entered.")
}
}
