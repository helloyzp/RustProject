//一个.rs文件天生就是一个模块(mod)，文件的名字就是模块的名字，因此hello.rs就是一个hello模块，mod hello;表示引用hello模块
mod hello;
//hello是文件
use hello::print_hello;//使用use引用hello这个模块下的print_hello()函数

mod printHello;//声明printHello这个模块
use printHello::hello2::print_hello2;//使用printHello这个模块的子模块hello2的print_hello2()函数

mod common;

mod ch03;
mod ch04;
mod ch05;
mod ch06;
mod ch07;
mod ch10;


fn main() {
    //println! 调用了一个 Rust 宏（macro）, 当看到符号 ! 的时候，就意味着调用的是宏而不是普通函数。
    //println!("Hello, world!");

    //ch03::basic_concepts::main();
    //ch04::ownership::main();
    //ch05::struct_demo::main();
    ch06::enums::main();

}

