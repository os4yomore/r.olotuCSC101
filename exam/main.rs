use std::io;
use std::io::Read;
use crate::io::stdin;
use std::io::Write;
use std::fs::File;

fn main() {
    println!("Welcome to the Lagoon Hospital, Lagos Database System.
If you're a Physician, Click 1
If you're a Patient, Click 2 ");

let mut input1 = String::new();
io::stdin().read_line(&mut input1).expect("Error");
let a:i32 = input1.trim().parse().expect("Invalid Input");

if a == 1{
    hospital_db();
}
if a == 2{
   patient();
}
}

fn hospital_db() {
println!("Welcome Doctor,
If you are Audu Gloria, click 1
If you are Seidu Ahmed, click 2
If you are Gbenga Mildred, click 3");
let  mut input1 = String::new();
io::stdin().read_line(&mut input1).expect("Error");
let a:i32 = input1.trim().parse().expect("Invalid Input");

if a == 1 || a == 2 || a == 3{
    println!("What's your area of specialization?
    If it's orthopedics, click 1,
    If it's surgery, click 2,
    If it's Pediatrics, click 3.");

let mut input2= String::new();
io::stdin().read_line(&mut input1).expect("Error");}

let b:i32 = input1.trim().parse().expect("Invalid Input");

if b == 1 || b == 2 || b == 3{
    let mut file = std::fs::File::open("rolotu_db.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);  }

 else {
    println!("Invalid User");
}
}

fn patient() {
    println!("Hello, what is your allergy?
    1- Low sugar
    2- Low cholesterol
    3- Diabetes
    4- Anaphylaxis
    5- Fish");
    
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Error");
    let a:i32 = input1.trim().parse().expect("Invalid Input");

    if a >= 3 {
        allergy_less_3();
    }
    else {
        allergy_greater_3();
    }
    
        
    
    
    
    
    fn allergy_less_3()  -> io::Result<()> {
     println!("A file has been generated");
let mut file = File::create("allergy.txt")?;
file.write_all("\n1	Agada	Simon	a-simon@gmail.com	12/3/1992	2	3
2	Fagbemi	Tina	f-tina@gmail.com	3/12/1989	3	1
3	Dalegi	Valerie	d-valerie@gmail.com	11/1/1993	1	2
4	Salami	Samuel	s-samuel@gmail.com	21/7/1998	2	5
5	Oghenero	Feji	o-feji@gmail.com	10/6/1991	3	2
6	Mustapha	Kabir	m-kabir@gmail.com	13/05/1990	2	4
7	Alokwe	Jane	a-jane@gmail.com	28/11/1988	3	1
8	Makman	Ali	m-ali@gmail.com	1/18/2000	1	3
9	Okonkwo	Chisom	o-chisom@gmail.com	15/11/1999	1	5
10	Eze	Agatha	e-agatha@gmail.com	22/04/1996	2	1
Allergies
1	Low Sugar	Feeling shaky or trembling
2	Low Cholesterol	Changes in your mood,sleeping, or eating patterns
3	Diabetes	Increased thirst".as_bytes()).expect("Failed to write");
Ok(()) }
  



}
fn allergy_greater_3 () -> io::Result<()> {
    println!("A file has been generated");
let mut file = File::create("allergy.txt")?;
file.write_all("\n
Patients
1	Agada	Simon	a-simon@gmail.com	12/3/1992	2	3
2	Fagbemi	Tina	f-tina@gmail.com	3/12/1989	3	1
3	Dalegi	Valerie	d-valerie@gmail.com	11/1/1993	1	2
4	Salami	Samuel	s-samuel@gmail.com	21/7/1998	2	5
5	Oghenero	Feji	o-feji@gmail.com	10/6/1991	3	2
6	Mustapha	Kabir	m-kabir@gmail.com	13/05/1990	2	4
7	Alokwe	Jane	a-jane@gmail.com	28/11/1988	3	1
8	Makman	Ali	m-ali@gmail.com	1/18/2000	1	3
9	Okonkwo	Chisom	o-chisom@gmail.com	15/11/1999	1	5
10	Eze	Agatha	e-agatha@gmail.com	22/04/1996	2	1
Allergies
4	Anaphylaxis	Rapid fall in blood pressure
5	Fish	Vomiting and diarrhea"
.as_bytes()).expect("Failed to write");
   
  Ok(()) 
}
