use std::io;
use rand::Rng;
use std::cmp::Ordering;

//结构体定义，只包含成员变量
struct MyRandom {
}

//结构体方法（注意与函数的叫法区别），在外部实现
impl MyRandom {
    fn rand_range(start:i32, end:i32)->i32{
        rand::thread_rng().gen_range(start..=end)
    }
}

fn main() {
    println!("Guessing the number!");

    //“方法_名字”或“功能_名字”的函数命名方式
   // let secret_number = rand::thread_rng().gen_range(1..101);
    //let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is {}", secret_number);

    let my_rand_number = MyRandom::rand_range(1,100);
    println!("my rand number is: {}", my_rand_number);


   loop {
       println!("Please input the number:");
       let mut input_number = String::new();
       io::stdin()
            .read_line(&mut input_number)
            .expect("Failed to read input");

        //println!("Your Guessing Number is: {}", input);

        //let input_number:i32 = input_number.trim().parse().expect("Please type a number");
        let input_number:i32 = match input_number.trim().parse(){
            Ok(num)=>num,//如果解析成功，这里将成功后结果返回，也就是parse函数的返回结果这里手动进行了处理
            Err(_)=>{
                println!("请输入数字！");
                continue;//这里跳转到下一次循环
            },
        };

        println!("Your Guessing Number is: {}", input_number);

        match input_number.cmp(&my_rand_number) {
            Ordering::Less => println!("Less"),
            Ordering::Equal =>{
                println!("Equal you win!");
                break; 
            } 
            Ordering::Greater => println!("Less"),
        };
    }
}
