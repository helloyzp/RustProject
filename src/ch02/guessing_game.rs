
//为了获取用户输入并打印结果作为输出，我们需要将 io（输入/输出）库引入当前作用域。io 库来自于标准库（也被称为 std）
use std::io;
use rand::Rng;
use std::cmp::Ordering;



/**


 运行：cargo run --bin guessing_game

*/
fn main() {

    //guess1();
    //guess2();
    //guess3();
    //guess4();
    guess5();
}

fn guess1() {
    //println! 是一个在屏幕上打印字符串的宏：
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();


    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

fn guess2() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //parse()将读取到的 String 转换为一个真正的数字类型，才好与秘密数字进行比较。
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

/**
    使用循环来允许多次猜测
*/
fn guess3() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //parse()将读取到的 String 转换为一个真正的数字类型，才好与秘密数字进行比较。
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}


/**
    使用循环来允许多次猜测
    猜测正确后退出
 */
fn guess4() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //parse()将读取到的 String 转换为一个真正的数字类型，才好与秘密数字进行比较。
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}


/**
    使用循环来允许多次猜测
    猜测正确后退出
    处理无效输入

    为了进一步改善游戏性，不要在用户输入非数字时崩溃，需要忽略非数字，让用户可以继续猜测。
    可以通过修改 guess 将 String 转化为 u32 那部分代码来实现。

    将 expect 调用换成 match 语句，是从遇到错误就崩溃转换到真正处理错误的惯用方法。
    须知 parse 返回一个 Result 类型，而 Result 是一个拥有 Ok 或 Err 成员的枚举。
    这里使用的 match 表达式，和之前处理 cmp 方法返回 Ordering 时用的一样。

    如果 parse 能够成功的将字符串转换为一个数字，它会返回一个包含结果数字的 Ok。
    这个 Ok 值与 match 第一个分支的模式相匹配，该分支对应的动作是返回 Ok 值中的数字 num，最后如愿变成新创建的 guess 变量。

    如果 parse 不能将字符串转换为一个数字，它会返回一个包含更多错误信息的 Err。Err 值不能匹配第一个 match 分支的 Ok(num) 模式，
    但是会匹配第二个分支的 Err(_) 模式：_ 是一个通配符值，本例中用来匹配所有 Err 值，不管其中有何种信息。
    所以程序会执行第二个分支的动作，continue 意味着进入 loop 的下一次循环，请求另一个猜测。
    这样程序就有效的忽略了 parse 可能遇到的所有错误！

 */
fn guess5() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /*

        //parse()将读取到的 String 转换为一个真正的数字类型，才好与秘密数字进行比较。
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        */

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}