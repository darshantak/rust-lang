mod final;

enum IpAddrKind {
    // V4(String),
    V4(u8,u8,u8,u8),
    V6(String)
}

struct IpAddr{
    kind : IpAddrKind,
    address : String
}

//The benefit of the enums is that we can define all the structs inside this enum.
enum Message { 
    Quit, //this variant contains no data 
    Move { x:i32,y:i32}, //this variant contains a struct
    Write(String),   //this varaint contains a string
    ChangeColor(i32,i32,i32,i32) //this variant contains 3 strings
}
//associted function with enum

impl Message{
    fn some_func(){
        println!("This is associated function with Message enum")
    }
}

//we can define isolated structs same as we defined inside enums
struct QuitMessage;
struct MoveMessage{
    x:i32,
    y:i32
}
struct WriteMessage(String);
struct ChangeColorMessage(i32,i32,i32,i32);

fn main() {
    println!("Enumns and Pattern Matching");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let localhost = IpAddr{
    //     kind : IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };
    // let localhost = IpAddrKind::V4(String::from("127.0.0.1"));
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
    
}

