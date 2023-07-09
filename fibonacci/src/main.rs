use std::io;

fn main() -> io::Result<()> {
    println!("Enter a positive, unsigned integer");
    
    let num: u32 = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let _input: u32 = match input.trim().parse() {
            Ok(num) => {
                break num;
            },
            Err(_) => {
                println!("Input a number");
                continue;
            }
        };
    };
    println!("========================================");

    for i in 0..=num {
        println!("Fibonacci number: {i}, Result: {}" , fibonacci(i));
    }
    Ok(())
}

fn fibonacci(number: u32) -> u32 {
    if number == 0 {
        return 0;
    } else if number == 1 {
        return 1;
    }

    return fibonacci(number - 1) + fibonacci(number - 2);
}
