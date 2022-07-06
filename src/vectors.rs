//Vectors - are Resizable arrays
use std::mem;

pub fn run() {
   let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5]; 
   
   //Re -assign value
   numbers[2] = 20;

   //Add on to vector
   numbers.push(7);
   numbers.push(8);

   println!("{:?}", numbers);


   //Get single value
   println!("single value: {}", numbers[0]);

   //Vector are stack allocated
   println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

   //Get slice
   let slice: &[i32] = &numbers[1..2];
   println!("slice: {:?}", slice);

   //loop through vector values
   for x in numbers.iter() {
    println!("Number: {}", x );
   }

  // loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
        println!("Numbers Vec: {:?}", numbers);
}