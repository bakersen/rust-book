fn main() {
    // if_else_statements();
    for_loop_range_rev();
}

fn if_else_statements(){
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //Using if in a let Statement
    let condition = true;
    let x:u32 = 5;
    let y:u32 = 6;
    let number = if condition {x} else {y};
    println!("The value of number is: {number}");
}

fn rust_loops(){
   loop {
        println!("again!");
    } 
}

fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50, 70, 60, 100, 38, 5699, 228];
    for element in a {
        println!("the value is: {element}");
    }
}

fn for_loop_range_rev() {
    for number in (1..10).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}