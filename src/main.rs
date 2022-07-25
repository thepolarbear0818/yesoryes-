#![allow(non_snake_case)]

use std::io;

fn check_yes(current_yes: String){
    let yes = "yes";
    let Yes = "Yes";
    let yEs = "yEs";
    let yeS = "yeS";
    let YES = "YES";
    let YEs = "YEs";
    let yES = "yES";
    let YeS = "YeS";

    if current_yes.trim() == yes{
        println!("");
        println!("That is a yes");
    }
    else if current_yes.trim() == Yes {
        println!("");
        println!("That is a Yes");
    }
    else if current_yes.trim() == yEs {
        println!("");
        println!("That is a Yes");
    }
    else if current_yes.trim() == yeS {
        println!("");
        println!("That is a Yes");
    }
    else if current_yes.trim() == YES {
        println!("");
        println!("That is a Yes");
    }
    else if current_yes.trim() == YEs {
        println!("");
        println!("That is a Yes");
    }
    else if current_yes.trim() == yES {
        println!("");
        println!("That is a Yes");
    }
    else if current_yes.trim() == YeS {
        println!("");
        println!("That is a Yes");
    }
    else{
        println!("That is not a yes");
    }



}

fn main() {
    let mut total_number = String::new();
    println!("Total Number of Input Statements");
    io::stdin()
    .read_line(&mut total_number)
    .expect("Failed to read line");
    let total_number: i32 = total_number.trim().parse().expect("Input not an integer");

    for i in 0..total_number{
        let mut input_yes = String::new();
        println!("Input your {} value",i);
        io::stdin()
        .read_line(&mut input_yes)
        .expect("Failed to read line");

        check_yes(input_yes);
        
    }

}


//have a user inputed number to say how many there are 
//have a list of things that could be eaqual to
//check each value to make sure that it matches against all of the values
//return yes or no after each test
