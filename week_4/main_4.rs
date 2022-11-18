fn main() {
   let fullname = "Chidubem John Umeh";
   let department = "Computer Science";
   let uni = "Pan-atlantic unversity";
   
   let mut school = "School of science".to_string();
   //push string
   school.push_str(" AND Technology");

println!("My name is: {}", fullname);
//check length
println! ("The length of my full name is: {}", fullname.len());
println!("I am a student of {} Department", department);
println!("{}", school);
println!("{}", uni);
}
