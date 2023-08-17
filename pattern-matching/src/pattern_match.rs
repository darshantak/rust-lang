fn main(){
    
}

fn multiply(i:Option<i8>) -> Option<i8>{
    match i {
        Some(x) => Some(x+1),
        None => None
    }
}