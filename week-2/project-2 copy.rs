fn main () { 
let a:f64 = 450_000.00 * 2.00;
let b:f64 = 1_500_000.00;
let c:f64 = 750_000.00 * 3.00;
let d:f64 = 2_850_000.00 * 3.00;
let e:f64 = 250_000.00;
let sum = a + b + c + d + e;
println! ("The sum of Toshiba, Mac, HP, Dell, Acer is {}", sum);
let average = sum / 10.00;
println! ("The average is {}", average);
}