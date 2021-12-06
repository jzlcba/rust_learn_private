struct User {
    name: String,
    addr: String,
    age: u8,
}

fn get_user_instance(name: String, addr: String, age: u8) -> User {
    User {
        name,
        addr,
        age,  
    }
}

fn main() {
    let st = User {
        name: String::from("张三"),
        addr: String::from("北京"),
        age: 22,
    };

    println!("Name {}, Addr {}, Age {}", st.name, st.addr, st.age);

    let name = String::from("李四");
    let addr = String::from("北京");

    let st = get_user_instance(name,addr,18);
    println!("Name {}, Addr {}, Age {}", st.name, st.addr, st.age);

    let st1 = User {
        name: String::from("王五"),
        addr: st.addr,
        age: st.age, 
    };
    println!("Name {}, Addr {}, Age {}", st1.name, st1.addr, st1.age);

    let st1 = User {
        name: String::from("王五五"),
        ..st1
    };
    println!("Name {}, Addr {}, Age {}", st1.name, st1.addr, st1.age);
}
