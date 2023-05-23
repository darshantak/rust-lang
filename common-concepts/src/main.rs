fn main() {
    //mutability of variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x= 8;
    println!("The value of x is: {}", x);

    //constant values
    const TEMP_NUMBER:u32 = 10000;
    println!("The value of constant value is {}",TEMP_NUMBER);
     
    //Shadowing
    let y = 5;
    println!("The value of y is {}",y);
    let y = 7;
    println!("The value of x is {}",y);
    let y = "abc";
    println!("The value of x is {}",y);

    //Compound types
    //tuples
    let tup = ("Hi, I am a Rusty now", 1000_00);
    let (_first,_second) = tup;
    let sub_vaalue = tup.1;
    println!("The second value is {}",sub_vaalue);
    //arrays
    let error_codes = [200,404,500];
    let not_found = error_codes[1];
    println!("getting values from array {}",not_found);

    let byte_array = [0;8]; //create an array with 8 elements and all are "0"
    println!("{:?}",byte_array);

    let sum = my_function(4,5);
    println!("The add function results in {}",sum);

    control_flows(14);

    loops();
}

fn my_function(x:i32,y:i32) -> i32{
    println!("Another function called ");
    println!("Argumets are {} and {}", x,y);
    let sum = x+y;
//    return sum;
    sum // or we can write x+y
}

fn control_flows(x:i32){
    if x>10{
        println!("Number {} is greater than 10",x);
    } else if x<10{
        println!("Number {} is smaller than 10",x);
    }else{
        println!("Number {} is equal to 10",x);
    }

    let condition = true;
    let number = if condition {5} else {6};
    println!("The 1 line condition returns us {}",number);

}

fn loops (){
    let mut counter = 0;
    
    println!("This is the first type of loop");
    let result = loop {
        counter += 1;
        if counter == 10{
            break counter;
        }
    //break; // this is important as this will go in infinite loop.
    };
    println!("This is how you return something with the help of first loop. Result is {}", result);

    println!("This is the generic while loop");
    while counter != 20{
        println!("Value of counter is {}",counter);
        counter +=1;
    }

    println!("This is the FOR loop");
    let a = [10,20,30,40,50];

    for element in a.iter(){
        println!("{}", element);
    }

    for element in 1..5{
        println!("{}",element);
    }
}