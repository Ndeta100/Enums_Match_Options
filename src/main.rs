enum IpAddrKind {
    v4(String),
    v6(String),
}
enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
    ChangeColor(u32, u32, u32),
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn route(ip_kind: IpAddrKind) {}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter {:?}", state);
            25
        }
    }
}
//Matching with option<T>

fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;
    let penny = Coin::Penny;
    let nickel = value_in_cents(penny);
    let alaska = Coin::Quarter(UsState::Alaska);
    let results = value_in_cents(alaska);
    println!("value in nickel is {:?}", results);
    // let home =IpAddr{
    //     kind:IpAddrKind::v4,
    //     address:String::from("Katusepapi")
    // };
    let home = IpAddrKind::v4(String::from("127.0.0.1"));
}
