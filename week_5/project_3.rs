use std::io;
fn main() {
   println!("MENU                  
   
  Key     Food Items                             Price
   1.   Poundo Yam & Edikaiko Soup            - N3_200.00
   2.   Fried Rice & Chicken                  - N3_000.00
   3.   Amala & Ewedu Soup                    - N2_500.00
   4.   Eba & Egusi Soup                      - N2_000.00
   5.   White Rice & Stew                     - N2_500.00

  Orders above N10,000 get a 10% discount");

   let mut input1 = String::new();
   println!("Enter the key for what you would like to have: ");
   io::stdin().read_line(&mut input1).expect("Not a valid string");
   let input1:i32 = input1.trim().parse().expect("Not a valid number");

   if input1 == 1 {
      println!("Your order is Poundo yam and Edikaiko soup");
      println!("How many portions of this meal would you like to order?");
      let mut input2 = String::new();
      io:: stdin().read_line(&mut input2).expect("Not a valid string");
      let c:f32 = input2.trim().parse().expect("Not a valid number");
      let d = 3200.00 * c;
      let discount:f32 = (d * 5.00)/100.0;
      let g:f32 = d - discount;
      if d  < 10_000.00 {
      println! ("Your total value of your meal is N{}", d); }
      else { println! ("The total value of your meal is N{}", g);}
   }
if input1 == 2 {
   println!("Your order is Fried Rice and Chicken");
   println!("How many portions of this meal would you like to order?");
   let mut input2 = String::new();
   io:: stdin().read_line(&mut input2).expect("Not a valid string");
   let c:f32 = input2.trim().parse().expect("Not a valid number");
   let r = 3000.00 * c;
   let discount2:f32 = (r* 5.00)/100.0;
   let b:f32 = r - discount2;
   if b  < 10_000.00 {
   println! ("Your total value of your meal is N{}", r); }
   else { println! ("The total value of your meal is N{}", b);}
}
if input1 == 3 {
   println!("Your order is Amala and Ewedu soup");
      println!("How many portions of this meal would you like to order?");
      let mut input2 = String::new();
      io:: stdin().read_line(&mut input2).expect("Not a valid string");
      let c:f32 = input2.trim().parse().expect("Not a valid number");
      let j = 2500.00 * c;
      let discount3:f32 = (j * 5.00)/100.0;
      let w:f32 = j - discount3;
      if j < 10_000.00 {
      println! ("Your total value of your meal is N{}", j); }
      else { println! ("The total value of your meal is N{}", w);}
}
if input1 == 4 {
   println!("Your order is Eba and Egusi Soup");
   println!("How many portions of this meal would you like to order?");
   let mut input2 = String::new();
   io:: stdin().read_line(&mut input2).expect("Not a valid string");
   let c:f32 = input2.trim().parse().expect("Not a valid number");
   let h = 2000.00 * c;
   let discount4:f32 = (h * 5.00)/100.0;
   let l:f32 = h - discount4;
   if h  < 10_000.00 {
   println! ("Your total value of your meal is N{}", h); }
   else { println! ("The total value of your meal is N{}", l);}
}
if input1 == 5 {
   println!("Your order is White Rice and Stew");
      println!("How many portions of this meal would you like to order?");
      let mut input2 = String::new();
      io:: stdin().read_line(&mut input2).expect("Not a valid string");
      let c:f32 = input2.trim().parse().expect("Not a valid number");
      let m = 2500.00 * c;
      let discount5:f32 = (m * 5.00)/100.0;
      let n:f32 = m - discount5;
      if m < 10_000.00 {
      println! ("Your total value of your meal is N{}", m); }
      else { println! ("The total value of your meal is N{}", n);}
}

}
