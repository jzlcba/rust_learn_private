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

    let mut x:u32 = 1;
    let x = loop {
        x += 1;
        if x == 30 {
            break x;
        }
    };
    println!("the value of x is {}", x);

    let arr = [1,2,3,4,5,6];
    for i in arr {
        println!("the value of i is {}", i);
    }

    for i in (2..7).rev() {
        println!("the value of i is {}", i);
    }

    for i in arr.iter().rev() {
        println!("the value of i is {}", i);
    }

    let s = String::from("s: &str");
    println!("the value of s is {}", s);

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid

    let x = 5;
    let y = x;
    println!("the value of y is {} is",y);

    let s1 = String::from("test");
    let s2 = s1.clone();
    println!("the value of s2 is {}", s1);
    println!("the value of s2 is {}", s2);
}
