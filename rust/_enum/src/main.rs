//定义一个枚举,具有两个枚举值V4和V6
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

//借助结构体和枚举，表达IPV4和6
#[derive(Debug)]
struct IpAddr{
    kind: IpAddrKind,
    address: String,
}


fn main() {

    let four = IpAddrKind::V4;//枚举类型，值为V4
    let six = IpAddrKind::V6;//枚举类型，值为V6
    println!("{:?}",four);
    println!("{:?}",six);
    
    let ip1 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let ip2 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:?}",ip1);
    println!("{:?}",ip2);

#[derive(Debug)]
enum IpAddrNew{
        V4(String),
        V6(String),
    }
    
    let ip1 = IpAddrNew::V4(String::from("127.0.0.1"));
    let ip2 = IpAddrNew::V6(String::from("::1"));
    println!("{:?}",ip1);
    println!("{:?}",ip2);

#[derive(Debug)]
struct IpAddrtest{
        ip1:(u8,u8,u8),
        ip2:String,
    }

    let st1 = IpAddrtest{
        ip1: (1,1,1),
        ip2: String::from("张三"),
    };

    println!("{:?}",st1);


#[derive(Debug)]
enum IpAddTest2 {
        Ip1(u8,u8,u8,u8),
        Ip2(String),
    }

    let ip1 = IpAddTest2::Ip1(1,2,2,2);
    let ip2 = IpAddTest2::Ip2(String::from("127.0.0.1"));
    println!("{:?}",ip1);
    println!("{:?}",ip2);

    // enum Option<T>
    // {
    //     None,
    //     Some(T),
    // }

    let v1 = Some(5);
    println!("{:?}",v1);
}
