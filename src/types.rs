/*
Primitive Types --
Integers: u8, i8, u16, i16, u32, i32, u64,i64, u128,i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at complie time, however, the compiler can usually infer what type we want to use based on the value and how we use it.


pub fn run() {
   let x = 1; //default is i32

   let y = 2.5; //by default "f64"

   //add explicit type
   let z: i64 = 45667888;

   //find max size
   println!("Max i32: {}", std::i32::MAX);
   println!("Max i64: {}", std::i64::MAX);

   //boolean
   let is_active: bool = true;

   //get boolean from expression
   let is_greater = 10 < 5;

   //char
   let a1 = 'a'; //for charater unicode
   let face = '\u{1F600}';


   println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}