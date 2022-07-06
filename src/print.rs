pub fn run() {
    //print to console
    println!("Hello from the print.rs file");
    //basic formating
    println!("Number: {}", 1);
    println!("{} is a good {}", "Yemi", "Man");
   //positional formating
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "mass","code");

   // Named Arguments
   println!("{name} likes to play {activity}", name = "Peter", activity = "baseball");

   //placeholder traits
   println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10 );

   //placeholder for debug trait
   println!("{:?}", (12, true, "hello"));

   //basic math
   println!("40 - 10 = {}", 40 - 10);
}