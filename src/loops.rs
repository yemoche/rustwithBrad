//Loops - They are used to iterate until a condition is met.


pub fn run() {
    //let mut x = 0;
    //infinite loop, we need to use a break to stop it
    // loop {
    //    x += 1;
    //    println!("Number: {}", x);

    //    if x == 20 {
    //     break;
    //    }
    // }
    // while loop (FizzBuzz)
    //    while x <= 100 {
    //      if x % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if x % 3 == 0 {
    //         println!("fizz");
    //     } else if x % 5 == 0 {
    //         println!("buzz")
    //     } else {
    //         println!("{}", x)
    //     }
    //     x += 1;
    //    }

    // For range
    for mut x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", x)
        }
        x += 1;
    }
}