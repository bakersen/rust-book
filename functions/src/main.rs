use std::io;

fn main() {
    get_array_index([100, 1121, 2000, 39393, 45656]);
}

fn get_array_index(a: [u32; 5]) {
    println!("Enter index");

    //Variable takes in input
    let mut input = String::new();

    //Call IO method
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    //Turn input into a string
    let index: usize = input.trim().parse().expect("Not a number");

    match check_array_len(a, index) {
        Return::Success(()) => {
            //print result
            println!("The array index is {}", a[index]);
        }
        Return::Error(msg) => {
            println!("Error: {}", msg);
        }
    }
}

enum Return {
    Success(()),
    Error(&'static str),
}

fn check_array_len(a: [u32; 5], index: usize) -> Return {
    if index >= a.len() {
        Return::Error("Index is out of bounds")
    } else {
        Return::Success(())
    }
}
