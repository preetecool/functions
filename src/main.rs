fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');
    statements_expressions();
    let five = five();
    println!("The value of five is: {five}");

    let plus_one = plus_one(5);

    println!("The value of plus_one is: {plus_one}");
}


// Functions in rust start with the fn keyword and use snake case for naming
// Parameters are special variables that are part of a function's signature. In function signatures, the declaration of the name and type of each parameter is a must.
fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {x}");

}
// When defining multiple parameters separate the parameters declarations with commas
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Statements and Expressions
fn statements_expressions(){
// Statements are intructinos that perform some action and do not return a value
// Expressions evaluate to a resultant value.
let y = 6; // statement 

// let x = (let y = 6);
// In other languages such as C or Ruby the above statement would be valid as the assignement operation returns a value. However, in Rust this is not the case. 

// x = y = 6 in those languages evaluates to x and y being assigned the value of 6. In Rust, this is not the case. The above statement is invalid as the let y = 6 statement does not return a value.

//Expressions evaluate to a value and make up most the rest of the code that you'll write in Rust. Consider the following example:

//Expression block starts
let y = { 
    let x = 3;
    x + 1 // <-- does not include a semi-colon (;) at the end of the expression
            // if we add a semi-colon at the end of the expression, it becomes a statement.
}; //Expression block ends

println!("EXPRESSION: The value of y is: {y}");

}

// Functions with Return Values


//
fn five() -> i32 {
    5 // <-- no semi-colon at the end of the expression

}

fn plus_one(x: i32) -> i32 {
    x + 1 // <-- no semi-colon at the end of the expression
            // adding a semi-colon at the end of the expression returns an error: error[E0308]: mismatched types
            //because the return type of the function is i32 and the semi-colon turns the expression into a statement which does not return a value.
}









