fn main() {
    let widgth = 10;
    let heigth = 10;

    println!("rectangle is {}", area(widgth, heigth));

    //let tuple:(i32,i32) = (10,10);
    let tuple = (10,11);
    println!("rectangle is {}", area1(tuple));
    println!("rectangle is {}", tuple.0 * tuple.1);

    let tuple = (String::from("张三"),11);
    {
        let tuple1 = tuple;
        println!("user is {:?}",tuple1);
        println!("user is {}:{}",tuple1.0,tuple1.1);
    }
    //println!("user is {}:{}",tuple.0,tuple.1);//tuple已经移动到了tuple1，所以这里不可用

    let tuple = (String::from("张三"),11);
    area3(tuple);//这里由于元组元素不是标量，所以执行的不是copy，而是move，如果不想拿走tuple的所有权，可以用切片引用或叫借用
    //println!("user is {}:{}",tuple.0,tuple.1);//由于上一句代码是对元组是move，所以这里元组已经不可再调用

    let st1 = Area {
        widgth: 11,
        heigth: 11,
    };

    println!("rectangle is {}",area2(&st1));
    println!("rectangle is {}",st1.widgth * st1.heigth);
    println!("rectangle is {}",st1.area());
    println!("rectangle is {:?}",st1);

    let r1 = Area::square(13);
    println!("rectangle is {:?}",r1);

    dbg!(st1);
}

fn area(widgth: i32, heigth: i32) -> i32 {
    widgth * heigth
}

fn area1(tuple: (i32,i32)) -> i32 {
    tuple.0 * tuple.1
}

#[derive(Debug)]
struct Area {
    widgth: i32,
    heigth: i32,
}

impl Area {
    //定义方法
    fn area(&self)->i32 {
        self.widgth * self.heigth
    }

    //定义不需要实例自身的方法,实例是点不出来的，只能用::调用
    fn square(size: i32) -> Area {
        Area{
            widgth: size,
            heigth: size}
    }
}

fn area2(rectangle: &Area) -> i32 {
    rectangle.widgth * rectangle.heigth
}

fn area3(tuple: (String,i32)){
    println!("user is {}:{}",tuple.0,tuple.1);
}