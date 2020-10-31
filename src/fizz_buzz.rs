pub fn run(num: i32) {
    if num % 3 == 0 && num % 5 == 0 {
        println!("Fizz Buzz")
    } else if num % 3 == 0 {
        println!("Fizz")
    } else if num % 5 == 0 {
        println!("Buzz")
    } else {
        println!("Not Fizzy or Buzzy")
    }
}
