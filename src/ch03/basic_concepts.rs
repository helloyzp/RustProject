pub fn main() {

    //variable();
    //variable2();
    //tupleDemo();
    //loop_break();
    //test_while();
    test_for();
}


/**
    不能对不可变变量 x 二次赋值（cannot assign twice to immutable variable x）
*/
fn variable() {
    let x = 5;
    println!("The value of x is: {}", x);
    //x = 6; //cannot assign twice to immutable variable
    println!("The value of x is: {}", x);
}

/**
    通过 mut，允许把绑定到 x 的值从 5 改成 6
*/
fn variable2() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

/**
    隐藏（Shadowing）

    可以用相同变量名称来隐藏一个变量，以及重复使用 let 关键字来多次隐藏
*/
fn shadow() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

// 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。
// 元组长度固定：一旦声明，其长度不会增大或缩小。
fn tupleDemo() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}

/**
    函数也可以被定义为拥有 参数（parameters），参数是特殊变量，是函数签名的一部分。
    当函数拥有参数（形参）时，可以为这些参数提供具体的值（实参）。
    parameter：形参，函数的参数
    argument：实参，调用函数时传入的具体值
*/
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

/**
    具有返回值的函数
*/
fn five() -> i32 {
    5
}

/**
    从循环返回
*/
fn  loop_break() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn test_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

/**
    作为更简洁的替代方案，可以使用 for 循环来对一个集合的每个元素执行一些代码。
    更为重要的是，我们增强了代码安全性，并消除了可能由于超出数组的结尾或遍历长度不够而缺少一些元素而导致的 bug。
    for 循环的安全性和简洁性使得它成为 Rust 中使用最多的循环结构。
*/
fn test_for() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}