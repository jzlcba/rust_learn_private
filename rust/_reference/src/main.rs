fn main() {
    let s = String::from("hello world");
    println!("s is {}", s);
    let pos = get_char_pos(&s);
    println!("pos is {}", pos);

    let s1 = &s[0..3];//0可以省略如s[..3]，
    let s2 = &s[3..s.len()];//s.len()可以省略如s[3..]，如果前后都省略，那么就是引用整个字符串，如[..]
    println!("s1 is \"{}\", and s2 is \"{}\"", s1, s2);

    let s_slice = get_char_slice(&s);

    println!("the s_slice is \"{}\"", s_slice);


    let s = "hello world";

    let s = get_char_slice_by_slice(&s[..]);
    println!("the s is \"{}\"", s);

    let arr = [1,2,3,4,5,6,7,8,9,10,11,12,13];
    let arr_s1 = &arr[..6];
    println!("the arr is {:?}",arr);
    println!("the arr_s1 is {:?}",arr_s1);
}

fn get_char_pos(s:&String) -> usize {
    let s_bytes = s.as_bytes();//这里使用字节数组来表示字符串
    for (i,&c) in s_bytes.iter().enumerate() {//这里使用了引用来表示某个字节的值
        if c == b' ' {
            return i;
        }
    }

    s.len()
}

fn get_char_slice(s:&String) -> &str {
    let s_bytes = s.as_bytes();//这里使用字节数组来表示字符串
    for (i,&c) in s_bytes.iter().enumerate() {//这里使用了引用来表示某个字节的值
        if c == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn get_char_slice_by_slice(s:&str) -> &str {
    let s_bytes = s.as_bytes();//这里使用字节数组来表示字符串
    for (i,&c) in s_bytes.iter().enumerate() {//这里使用了引用来表示某个字节的值
        if c == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
