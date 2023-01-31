use std::io;
 
fn age18() {
println!("What is this siblings name?");
let mut name = String::new();
let _z = std::io::stdin().read_line(&mut name).unwrap();
        
    println!("What is this siblings age?");
        let mut ages = String::new();
        io::stdin().read_line(&mut ages).expect("Input failed to parse");
        let t:i32 = ages.trim().parse().expect("Invalid input");

        println!("Your sibling is married. Type 1 if this statement is true, and 0 if it false.");
        let mut mar = String::new();
        io::stdin().read_line(&mut mar).expect("Input failed to parse");
        let a:i32 = mar.trim().parse().expect("Invalid input");

    if a == 1{
        println!("This sibling is married, now, what city do they live in?");
        let mut city = String::new();
        let _b = io::stdin().read_line(&mut city).unwrap();
    
        println!("Does this married sibling have children? Click 1 for yes, and 0 for no.");
        let mut offsprings = String::new();
        io::stdin().read_line(&mut offsprings).expect("Input failed to parse");
        let c:i32 = offsprings.trim().parse().expect("Invalid input");
    
        if c == 1{
        
            println!("The name of this sibling is {}, and they are {} years old. They are married, live in {}, and have children.", name.trim(), t, city.trim());
    
        }
        else{
            println!("The name of this sibling is {}, and they are {} years old. They are married, live in {}, and have no children.", name.trim(), t, city.trim());
        }}

    else if a == 0{
        println!("Your sibling is single. If your sibling is a student, click 1, however if your sibling is a worker, click 2.");
        let mut sw = String::new();
        io::stdin().read_line(&mut sw).expect("Input failed to parse");
        let d:i32 = sw.trim().parse().expect("Invalid input");
    
        if d == 1{
            println!("What University do they study at?");
            let mut uni = String::new();
            let _y = std::io::stdin().read_line(&mut uni).unwrap();
            
           
            
            println!("What course do they study?");
            let course = String::new();
            let _r = std::io::stdin().read_line(&mut name).unwrap();

            println!("The name of this sibling is {}, they are {} years old, single, and study {} at {}.", name.trim(), t, course.trim(), uni.trim());
        }

        else{
            println!("The name of this sibling is {}, they are {} years old, and they are a single worker.", name.trim(), t);
        }
    }
}

   
 

 fn below18() {
println!("What is this siblings name?");
let mut name = String::new();
let _z = std::io::stdin().read_line(&mut name).unwrap();

println!("What is this siblings age?");
        let mut ages = String::new();
        io::stdin().read_line(&mut ages).expect("Input failed to parse");
        let t:i32 = ages.trim().parse().expect("Invalid input");

    
println!("Have they written WAEC? Click 1 for yes, and 0 for no.");
    let mut waec = String::new();
        io::stdin().read_line(&mut waec).expect("Input failed to read");
        let g:i32 = waec.trim().parse().expect("Input invalid");

        if g == 1{
            println!("What secondary school did they attend?");
            let mut sschool = String::new();
            let _h = io::stdin().read_line(&mut sschool).unwrap();

            println!("The name of this sibling is {}, they are {} years old, they have already written WAEC, and they attended {}.", name.trim(), t, sschool.trim());
        }
        else if g == 0{
            println!("What class are they currently in?");
            let mut class = String::new();
            let _i = io::stdin().read_line(&mut class).unwrap();

            println!("The name of this sibling is {}, they are {} years old, they have not written WAEC, and they are currently in {}.", name.trim(), t, class.trim());

        }
 }




 fn main(){
    println!("How many siblings do you have?");
    let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Input failed to parse");
        let upper_bound:i32 = number.trim().parse().expect("Invalid input");

        
        for _x in 0..upper_bound{
        println!("Is this sibling above 18? If yes, click 1, otherwise, click 0");
        let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Input failed to parse");
        let k:i32 = age.trim().parse().expect("Invalid input");

        if k == 1{
            age18();
        }
        else{
            below18();
        }


        }
 }
