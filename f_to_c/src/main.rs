use std::io;

fn main()  -> io::Result<()> {
    println!("Enter in degrees in Fahrenheight to convert to Celsius");

    let degree_f: f32 = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)?;

        let _input: i32 = match input.trim().parse() {
            Ok(num) => {
                break num;
            },
            Err(_) => {
                println!("Input a number");
                continue;
            }
        };
    };

    let degree_c = convert_f_to_c(degree_f);
    println!("Degree F: {degree_f} to Degree C: {degree_c}");
    Ok(())
}

fn convert_f_to_c(degree_f: f32) -> f32 {
    return (degree_f - 32.0) * (5.0 / 9.0);
}
