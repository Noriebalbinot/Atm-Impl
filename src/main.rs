use models::machine_model::AtmMachine;
use text_io::read;

mod machine;
mod models;

fn main() {
    let mut machine = AtmMachine::new();

    loop {
        println!("Welcome to The Atm Machine");
        println!("to use, just type one of the commands and press enter:");
        println!("d - enter in deposit mode!");
        println!("w - enter in withdraw mode!");
        let input: String = read!("{}");
        match input.as_str() {
            "d" => {
                println!("Welcome To Deposit Mode");
                println!("digit number to be deposit and let");
                println!("the algoritm decide the notes or just");
                println!("add the note value and press enter");
                loop {
                    let inputdeposit: String = read!("{}");
                    match inputdeposit.as_str() {
                        "exit" => {
                            break;
                        }
                        _ => match inputdeposit.parse::<i32>() {
                            Ok(a) => {
                                let bills = machine.deposit(a);
                                println!("{a} deposited:");
                                println!("{} x 100 dollars bills", bills.onehundred);
                                println!("{} x 50 dollars bills", bills.fifty);
                                println!("{} x 20 dollars bills", bills.twenty);
                                println!("{} x 10 dollars bills", bills.ten);
                                println!("{} x 5 dollars bills", bills.five);
                                println!("{} x 1 dollar bills", bills.one);
                            }
                            Err(_) => {
                                println!("Sorry to deposit a value you need to insert a valid integer number!")
                            }
                        },
                    }
                }
            }
            _ => {
                println!("Wrong command input!")
            }
        }
    }
}
