// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


//The problem here was that the function didn't return a value because of the ; at the end it was returning this '()' in order to fix that we have 2 choices
// First, add a return keyword at the line 17 to return theresult of the operation
// Second, just suppress the ; and it will return the value of the operation

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
     num * num
}
