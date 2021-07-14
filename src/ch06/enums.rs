enum IpAddrKind {
    V4,
    V6,
}

//我们使用了一个结构体来将 kind 和 address 打包在一起，现在枚举成员就与值相关联了。
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

pub fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;


    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    /*    let usStat:if_let::UsState = if_let::UsState::Alabama;
        let  coin : if_let::Coin = if_let::Coin::Quarter(usStat);
        if_let::test_coin2(coin);*/
}

fn route(ip_type: IpAddrKind) {}


fn use_ip_addr() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}


//一个 Message 枚举，其每个成员都存储了不同数量和类型的值
enum Message {
    Quit,
    //Quit成员没有关联任何数据
    Move { x: i32, y: i32 },
    //Move成员存储了两个i32类型的值
    Write(String),
    //Write成员存储了一个String类型的值
    ChangeColor(i32, i32, i32),   //ChangeColor成员存储了三个i32类型的值
}


//定义一个如示例 6-2 中所示那样的有关联值的枚举的方式和定义多个不同类型的结构体的方式很相像
struct QuitMessage;

// 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);

// 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体


//就像可以使用 impl 来为结构体定义方法那样，也可以在枚举上定义方法。
// 这是一个定义于我们 Message 枚举上的叫做 call 的方法
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}


fn test_enum_method() {
    //创建了一个值为 Message::Write(String::from("hello")) 的变量 m
    let m = Message::Write(String::from("hello"));
    m.call();
}


fn test_Option() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents2(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


//绑定值的模式
mod Patterns_that_Bind_to_Values {
    #[derive(Debug)] // 这样可以立刻看到州的名称
    pub(crate) enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    pub(crate) enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }


    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    fn test() {
        value_in_cents(Coin::Quarter(UsState::Alaska));
    }
}


//匹配 Option<T>
mod Matching_with_Option {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    fn test() {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }
}

//通配符_
mod The_Placeholder {
    fn test() {
        let some_u8_value = 0u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => (),
        }
    }
}


mod if_let {
    use crate::ch06::enums::{Patterns_that_Bind_to_Values, Patterns_that_Bind_to_Values::Coin, Patterns_that_Bind_to_Values::UsState};

    fn test() {
        let some_u8_value = Some(0u8);
        match some_u8_value {
            Some(3) => println!("three"),
            _ => (),
        }
    }

    fn test_if_let() {
        let some_u8_value = Some(0u8);
        if let Some(3) = some_u8_value {
            println!("three");
        }
    }

    fn test_coin1(coin: Patterns_that_Bind_to_Values::Coin) {
        let mut count = 0;
        match coin {
            Coin::Quarter(state) => println!("State quarter from {:?}!", state),
            _ => count += 1,
        }
    }

    pub(crate) fn test_coin2(coin: Coin) {
        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }
    }
}
