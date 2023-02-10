use std::io::Read;
use std::io;

fn main() {
    println!("Welcome to the Globacom Database Management System.
If you are an administrator, click 1
If you are a project manager, click2
If you are an employee, click 3
If you are are a customer, click 4
If you are a vendor, click 5");

let mut input1 = String::new();
io::stdin().read_line(&mut input1).expect("Not a valid string");
let a:i32 = input1.trim().parse().expect("Not a valid number");

if a == 1{
    admin();
}
else if a == 2{
    manager();
}
else if a == 3{
    employee();
}
else if a == 4{
    customer();
}
else if a == 5{
    vendor();
}
}

fn admin() {
    let mut file = std::fs::File::open("globacom_dbase.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn manager() {
    let mut file = std::fs::File::open("project_tb.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn employee() {
    let mut file = std::fs::File::open("staff_tb.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn customer() {
    let mut file = std::fs::File::open("customers_tb.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn vendor() {
    let mut file = std::fs::File::open("dataplan_tb.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

