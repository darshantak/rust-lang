fn main(){
 let v1 = Some(5);
 let v2 = Some(String::from("For Some option we don't annotaote the type"));
 let v3: Option<i32> = None;
 
 let x : i8 = 5;
 let y : Option<i8> = Some(4);
let z : Option<i8>= None;
 let sum = x+z.unwrap_or(0);
 println!("{}",sum);

 value_in_cents(Coin::Test(UsStates::Alaska));
}

//Matching patterns-> It allows us to match value against a set of pattern

#[derive(Debug)]
enum UsStates {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California
}
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
    Test(UsStates)
}

fn value_in_cents(coin:Coin) -> i8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::Test(state) => {
            println!("Test is called with state {:?}",state );
            1
        }
    }
}
