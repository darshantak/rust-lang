struct User {
    username : String ,
    emailId: String,
    sign_in_int : u64,
    active: bool
}

struct Rectangle {
    width : u32,
    height : u32
}
fn main(){
    let mut user1 = User{
        username: String::from("darshantak"),
        emailId: String::from("abc@xyz.com"),
        active: true,
        sign_in_int: 1
    };

    let name = user1.username;
    user1.username = String::from("takdarshan");

    let user2 = build_user(String::from("x@b.x"), String::from("takdarshan"));

    let user3 = User{
        emailId : String::from("new@new.com"),
        ..user2
    };
    //Creating tuple structs without the field names
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);

    //calling the area function with the tuple
    let rect = (40,10);
    println!("Area of the rectangle is {}",area(rect));

    //calling the area function with structs
    let rect_strut = Rectangle{
        width:10,
        height:100
    };
    println!("Area of the rectangle with structs is {}",area_struct(& rect_strut));
}

fn build_user(emailId:String, username:String) -> User{
    User{
        emailId, //emailId: emailId -> The args are same as the key names so this can be done.
        username, // this is known as the backhand
        active:true,
        sign_in_int:1,
    }
}

fn area(dimns:(u32,u32))->u32 {
    dimns.0*dimns.1
}

fn area_struct(dimns : &Rectangle) -> u32{
    dimns.width*dimns.height
}