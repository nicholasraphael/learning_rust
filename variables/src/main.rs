fn main() {
    // constants
    const MAX_POINTS: u32 = 10000;
    println!("Our max points are {}", MAX_POINTS);


    let x = 5;
    println!("The value of x is {}", x);

    // Shadowing example, making new variables with the same name
    let x = x + 2;
    println!("The value of y is {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    // compound types
    // tuples
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tuple values: {}, {}, {}", _tup.0, _tup.1, _tup.2);

    // arrays
    // giving array types
    let _a: [i32; 5] = [0; 5];

    println!("Array: {}", _a[0]);

    another_function(_tup.0, _tup.1);
    let called = function_returns();
    println!("function was called: {}", called);

    for i in (1..4).rev() {
        println!("{}", i);
    }
}

fn another_function(x: i32, y: f64) {
    println!("Function call with x = {} and y = {}", x, y);
}

fn function_returns() -> bool {
    true
}