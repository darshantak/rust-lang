fn main() {
{//s is not valid here
    let s = "hello world"; //s is declared now. We can use s going forward
    let s1 = String::from("hello");


}//the scope is now over for s
let get_strfn1 = get_string();
println!("got string = {}",get_strfn1);
ownership();

//references
let refers1 = String::from("Reference fn");
let referLength = calulate_length(&refers1);
println!("String is {} and length is {}",&refers1,referLength);
}

fn ownership(){
    let s1 = String::from("Hello");
    let s2 = s1; //Move(not shallow copy)
    let s3  = s2.clone(); //error bc the value of s1 is moved to s2

    println!("{}",  s3);
}

//Ownership and Fucntions 
fn get_string() -> String {
    let someStr = String::from("Hello");
    someStr
}


//refernces and borrowing
fn calulate_length(s: &String) -> usize {
    //putting & to get the reference of the variable to avoid the moving
    let length = s.len();
    length
}