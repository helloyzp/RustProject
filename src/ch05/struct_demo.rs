struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub(crate) fn main() {

    //tuple_struct();
    //printRect();
   //printRect2();
    test_method_syntax();
}

fn define_struct() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    //如何改变一个可变的 User 实例 email 字段的值：
    //注意整个实例必须是可变的；Rust 并不允许只将某个字段标记为可变。
    user1.email = String::from("anotheremail@example.com");
}

//同其他任何表达式一样，我们可以在函数体的最后一个表达式中构造一个结构体的新实例，来隐式地返回这个实例。
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

//变量与字段同名时的字段初始化简写语法
fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

//使用结构体更新语法从其他实例创建实例
//struct update syntax
fn update_struct() {

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };


    // 展示了不使用更新语法时，如何在 user2 中创建一个新 User 实例。
    // 我们为 email 和 username 设置了新的值，其他值则使用了 user1 中的同名值：
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };


    //使用结构体更新语法，我们可以通过更少的代码来达到相同的效果，
    //.. 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值。
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

}


//使用没有命名字段的元组结构体来创建不同的类型
//也可以定义与元组（在第三章讨论过）类似的结构体，称为 元组结构体（tuple structs）。
//元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。
fn tuple_struct() {
    //要定义元组结构体，以 struct 关键字和结构体名开头并后跟元组中的类型。
    // 例如，下面是两个分别叫做 Color 和 Point 元组结构体的定义和用法：
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(1, 2, 3);
    let origin = Point(1, 2, 3);
    //注意 black 和 origin 值的类型不同，因为它们是不同的元组结构体的实例

    println!("black.0={}, black.1={}, black.2={}" , black.0, black.1, black.2)


}

//没有任何字段的类单元结构体
fn unit_like_struct() {


}


fn calculate_rectangle_area() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

//使用元组重构: 使用元组来指定长方形的宽高
fn calculate_rectangle_area2() {
    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect)
    );
}

fn area2(dimensions: (u32, u32 )) -> u32 {
    dimensions.0 * dimensions.1
}

//使用结构体重构：赋予更多意义
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn calculate_rectangle_area3() {
    let rect = Rectangle{width:30, height:50};

    println!(
        "The area of the rectangle is {} square pixels.", area3(&rect)
    );
}

//参数rectangle，其类型是一个结构体Rectangle实例的不可变借用
//第四章讲到过，我们希望借用结构体而不是获取它的所有权，这样 main 函数就可以保持 rect 的所有权并继续使用它，所以这就是为什么在函数签名和调用的地方会有 &。
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn printRect() {
    let rect = Rectangle { width: 30, height: 50 };

    //println!("rect1 is {}", rect); //`{}` 默认告诉 `println!` 使用被称为 `Display` 的格式
}

fn printRect2() {
    let rect = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect);//{:?}指示符告诉 println! 我们想要使用叫做 Debug 的输出格式。
}



impl Rectangle {
    //在 Rectangle 结构体上定义 area 方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //在 Rectangle 上定义 can_hold 方法
    //在方法签名中，可以在 self 后增加多个参数，而且这些参数就像函数中的参数一样工作。
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //关联函数(Associated Functions)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

//测试方法语法(Method Syntax)
fn test_method_syntax() {
    let rect = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()  //方法语法获取一个实例并加上一个点号，后跟方法名、圆括号以及任何参数。
    );

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

//测试关联函数(Associated Functions)
fn test_associated_functions() {
   let square = Rectangle::square(50);
    println!("rect1 is {:?}", rect);
}