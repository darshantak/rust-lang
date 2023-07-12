// #[derive[Debug]]
struct Rectangle {
    width : u32,
    height: u32
}
impl Rectangle{
    fn area(&self) -> u32{
        self.width*self.height
    }

    fn compare(&self,other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }   
}
//associative functions
impl Rectangle{
    fn returnSquare(size:u32)-> Rectangle{
        Rectangle{
            width:size,
            height:size
        }
    }
}

fn main(){
    let rect = Rectangle{
        width:10,
        height:20
    };
    let rect1 = Rectangle{
        width:10,
        height:30
    };
    let rect2 = Rectangle{
        width:20,
        height:30
    };

    println!("Area is {:?}",rect.area());
    println!("{}",rect1.compare(&rect2));
    //calling the associative functions

    let rect3 = Rectangle::returnSquare(25);
}