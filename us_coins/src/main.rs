#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    println!("Hello, world!");
    let penny = Coin::Penny;
    value_in_cents(penny);
    // println!("{:#?}", penny);

    let quarter = Coin::Quarter(UsState::Alaska);
    // println!("{:#?}", quarter);
    value_in_cents(quarter);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
