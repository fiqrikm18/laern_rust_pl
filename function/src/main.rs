fn main() {
    println!("Hello, world!");
}

// note: that function with returning value not use `return` keyword
// instead just put the return value without semicolon
fn func_with_returning_date(x: i32) -> i32 {
    x
}

// function without returning value
fn print_data(x: i32) {
    println!("Print data {x}");
}
