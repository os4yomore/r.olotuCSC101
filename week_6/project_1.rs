use std::io;

 fn trap () {
println! ("What is the height of the trapezium? ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Nota avalid number");
println!("What is the length of base 1?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");
println!("What is the length of base 2?");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");

    let d = a / 2.00 * (c + b);
 println!("The area of the trapezium is {}", d);
 }

 fn rhom () {
    println! ("What is the length of the 1st diagonal? ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let e:f64 = input1.trim().parse().expect("Nota avalid number");
println!("What is the length of the 2nd diagonal?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let f:f64 = input2.trim().parse().expect("Not a valid number");

    let g = 0.50 * e * f;
 println!("The area of the rhombus is {}", g);
 }

 fn para () {
    println! ("What is the length of the base of the parallelogram? ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let h:f64 = input1.trim().parse().expect("Nota avalid number");
println!("What is the altitude of the parallelogram?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let i:f64 = input2.trim().parse().expect("Not a valid number");

    let j = h * i;
 println!("The area of the parallelogram is {}", j);
 }

 fn cube () {
    println! ("What is the lenght of a side of the cube? ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let k:f64 = input1.trim().parse().expect("Nota avalid number");
    
    let l = 6.00 * k * k;
 println!("The surface area of the cube is {}", l);
 }

 fn cylinder () {
    println! ("What is the radius of the cylinder? ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let m:f64 = input1.trim().parse().expect("Nota avalid number");
println!("What is the height of the cylinder?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let n:f64 = input2.trim().parse().expect("Not a valid number");


    let o = 3.14159265359 * m * m * n;
 println!("The volume of the cylinder is {}", o);
 }

 fn main () {
    println!("This is a program developed to perform the following calculations
    KEY                      CALCULATION
    1.                       Area of a trapezium
    2.                       Area of a rhombus
    3.                       Area of a parallelogram
    4.                       Surface area of a cube
    5.                       Volume of a cylinder ");

    println!("Type in the key of the caluclation you would like to perform!");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let z:i32 = input1.trim().parse().expect("Not a valid number");

    if z == 1 {
        trap();
    }
    if z == 2 {
         rhom();
    }
    if z == 3 {
        para();
    }
    if z == 4 {
        cube();
    }
    if z == 5 {
        cylinder();
    }












}