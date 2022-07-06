//Variables hold primitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language

pub fn run() {
   let name = "Brad";
   let mut age = 37;
   println!("my name is {} and my age is {}", name, age);
   age = 38;
   println!("my name is {} and my age is {}", name, age);

   //Define constant
   const ID: i32 = 001; //you must explicitly define your type when using cosnt
   println!("ID: {}", ID );

   //Assign multiple variable ones
   let ( my_name, my_age ) = ("Brad", 37);
   println!("{} is {} ", my_name, my_age);
}