use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
  let name_arr:[&str; 5]= ["Oluchi Mordi", "Adams Aliyu", "Shania Bolanle", "Adekunle Gold", "Bianca Edemoh"];
  
  let matric_arr:[&str; 5]= ["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];

  let dept_arr:[&str; 5]= ["Accounting", "Economics", "Computer Science", "Electrical Engineering", "Mechanical Engineering"];

  let level_arr:[i32; 5]= [300, 100, 200, 200, 100];

  for index in 0..5

  {println!(" Name: {:?}           
  Matriculation Number: {:?}
  Department: {:?}     
  Level: {:?}
        
         ", 
         
         name_arr[index],matric_arr[index],dept_arr[index],level_arr[index]
        
        );}
  
  


let mut file = File::create("Project2.txt")?;
file.write_all("Project 2\n".as_bytes()).expect("Failed to write");

for index in 0..5 {

  writeln!(file, "{}
  {}
  {}
  {}",name_arr[index],matric_arr[index],dept_arr[index],level_arr[index]
        
)?;
}

Ok(())

}