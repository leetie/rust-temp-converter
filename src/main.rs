use std::io;

fn main() {
    // get choice for conversion algorithm
    let choice: bool = get_conversion_choice();
    // get user input, pass that input to function to convert to i32, then call chosen conversion method with that number
    let input = get_temp_input(false);
    let input: i32 = temp_input_to_i32(input);

    if choice == true {
        println!("The conversion is: {}", f_to_c(input))
    } else {
        println!("The conversioin is: {}", c_to_f(input))
    };
}

fn f_to_c(temp: i32) -> i32 {
    (((temp as f64) - 32.0) * (5.0 / 9.0)) as i32
}

fn c_to_f(temp: i32) -> i32 {
    ((temp as f64) * (9.0 / 5.0) + 32.0) as i32
}
//  F->C true, C->F false
fn get_conversion_choice() -> bool {
    println!("Would you like to convert from C -> F? (1), or F -> C? (2)");
    loop {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("failed");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter 1 or 2");
                continue;
            }
        };
        if choice == 1 {
            break false;
        } else if choice == 2 {
            break true;
        } else {
            continue;
        }
    }
}
fn get_temp_input(repeat: bool) -> String {
    if repeat == true {
        println!("Please input a whole number eg 35")
    };
    let mut input = String::new();
    println!("Input number to be converted");
    io::stdin().read_line(&mut input).expect("Failed");
    input
}
fn temp_input_to_i32(mut input: String) -> i32 {
    let mut user_input_correctly = false;
    let mut output: i32 = 0;
    while user_input_correctly == false {
        output = match input.trim().parse() {
            Ok(int) => {
                user_input_correctly = true;
                int
            }
            Err(_) => {
                input = get_temp_input(true);
                continue;
            }
        };
    }
    output
}
