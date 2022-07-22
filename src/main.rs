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
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
fn route(ip_kind: IpAddrKind) {}
fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    // let home =IpAddr{
    //     kind:IpAddrKind::v4,
    //     address:String::from("Katusepapi")
    // };
    let home = IpAddrKind::v4(String::from("127.0.0.1"));
}
