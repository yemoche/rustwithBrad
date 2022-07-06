//primitive str = immutable fixed-length string somewhere in the memory
//String = Growable, heap-allocated data structure- Used when u need to modify or own string data

pub fn run() {
    // let hello = "hello"; // primitive type
   let mut hello = String::from("Hello ");

   //adding character
   hello.push('W'); //can work only with one char

    hello.push_str("orld"); //with the push_str, pushes more than one character

   //get length...it works for both primitive and String
   println!("Lenght: {}", hello.len());
  
   // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word );
    }

    //create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Asssertion testing
    assert_eq!(2, s.len()); //testing for length
    assert_eq!(11, s.capacity());

    println!("{}", s) ;
}