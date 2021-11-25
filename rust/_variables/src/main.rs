

fn main() {
    let x1 = 5;
    let x = x1 + 1;
    println!("the value of x = {}", x1);

    {
        let x = x * 2;
        println!("the value of x = {}", x);
    }
    println!("the value of x = {}", x);

    let _str = "lkasdjf";
    let _str = _str.len();

    let guess: i32 = "32".parse().expect("not a number");
    println!("the value of guess = {}", guess);

    let _tup:(i32, u32, String) = (1,2,"kajlsdf".to_string());
    println!("the value of x = {}", _tup.0);
    println!("the value of y = {}", _tup.1);
    println!("the value of z = {}", _tup.2);

    let _arr:[i32;6] = [1,2,3,4,5,6];
    
    let _arr = [3;5];
    println!("the value of _arr_0 = {}", _arr[0]);
    println!("the value of _arr_1 = {}", _arr[1]);


}