fn main() {
    
    let x = 5;

    if x < 5 {
        println!("the value of x is smaller than 5");
    }
    else if x == 5 {
        println!("the value of x is equal to 5");
    }
    else {
        println!("the value of x is lager than 5");
    }

    let num = if true {1} else {0};
    println!("the value of num is {:?}", num);
}
