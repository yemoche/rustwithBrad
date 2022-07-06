//Arrays - fixed where elements are the same data types
use std::mem;

pub fn run() {
   let mut numbers: [i32; 5] = [1, 2, 3, 4, 5]; //datatypes : i32, and length of 5, adding mut makes it changeable
   
   //Re -assign value
   numbers[2] = 20;

   println!("{:?}", numbers);


   //Get single value
   println!("single value: {}", numbers[0]);

   //Arrays are stack allocated
   println!("Array occupies {} bytes", mem::size_of_val(&numbers));

   //Get slice
   let slice: &[i32] = &numbers[1..2];
   println!("slice: {:?}", slice);

}