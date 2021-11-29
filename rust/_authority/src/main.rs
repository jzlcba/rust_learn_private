// fn main() {
//     let s = String::from("hello world");//堆上分配内存
//     let x:u32 = 5;//栈上分配内存

//     make_move(s);//s的值移动到形参s上

//     make_copy(x);//x的值拷贝到形参x上

//     //println!("s is {}", s);//由于调用make_move时s的值得所有权移动到了形参上，不会自动移动回来，所以这里编译失败了。
//     println!("x is {}",x);//由于x是拷贝，没有所有权的移动，所以x还是有效的
// }

// fn make_move(s: String){
//     println!("the string s is {}", s);
// }//形参的作用域结束，s指向的堆内存释放。

// fn make_copy(x:u32){
//     println!("the x is {}", x);
// }

// fn main() {
//     let s1 = gives_ownership();//函数内创建一个字符串，并将所有权传出
//     println!("the string s1 is {}", s1);

//     let s2 = String::from("other string on heap!");

//     println!("the string s2 is {}", s2);

//     let s3 = gives_and_back_ownership(s2);//函数外部创建的字符串，移动到函数内部，操作完成后将所有权交回
    
//     //println!("the string s2 is {}", s2);//由于所有权已经移动到了s3，s2无效
//     println!("the string s3 is {}", s3);//s3是有效的
// }

// fn gives_ownership()->String{
//     let s = String::from("a string on heap!");
//     s
// }

// fn gives_and_back_ownership(s: String)->String{
//     s
// }

// fn main() {
//     let mut s1 = String::from("a string on heap!");
//     let s1_len = make_ref(&mut s1);//可变引用必须指明，告诉读者函数内部要改这个变量
//     println!("the  s1 is: {}", s1);
//     println!("the len of s1 is: {}", s1_len);
// }

// fn make_ref(s:&mut String)->usize {
//     s.push_str("xxxx");
//     s.len()
// }


fn main() {
    let s = make_haning();
}

fn make_haning() -> &String {
    let s = String::from("aaaa");
    &s
}