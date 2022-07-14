fn main() {
    let x = 1; // immputable var
    let mut y = 5; // muttable var

    let x = x + 1; // shadowing var

    let tup: (i32, u32, &str) = (5, 6, "test"); // tuple
    let arr = [1, 2, 3]; // array
}
