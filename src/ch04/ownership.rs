pub fn main() {
    //test_move();
    //test_OwnershipandFunctions();
    //test_ReturnValuesAndScope();

    //test_find_first_word();
    test_find_first_word_use_slice();

}

/**
    变量与数据交互的方式（一）：移动


 */
fn test_move() {
    //    “将 5 绑定到 x；接着生成一个值 x 的拷贝并绑定到 y”。现在有了两个变量，x 和 y，都等于 5。
    //     这也正是事实上发生了的，因为整数是有已知固定大小的简单值，所以这两个 5 被放入了栈中。
    let x = 5;
    let y = x;
    println!("x={}, y={}", x, y);

    //现在看看这个 String 版本：
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}, world!", s1);
}


/**
    Ownership and Functions
    所有权与函数
*/
fn test_OwnershipandFunctions() {
    let s = String::from("hello");  // s 进入作用域

    //此时向函数传递值是移动(move)
    takes_ownership(s);             // s 的值移动到函数里 ...
                                               // ... 所以到这里不再有效

    //println!("{}, world!", s); //这行代码编译会报错，因为变量s的所有权已经移动给了takes_ownership()函数的some_string变量

    let x = 5;                            // x 进入作用域

    //此时向函数传递值是复制(copy)
    makes_copy(x);                 // x 移动函数里，
                                               // 但 i32 是 Copy 的，所以在后面可继续使用 x

    println!("x={}", x); //变量x这里依然可以正常使用，因为x是i32，是Copy的


}// 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作


/**
    返回值与作用域
*/
fn test_ReturnValuesAndScope() {
    let s1 = gives_ownership();                  // gives_ownership 将返回值移给 s1

    println!("s1={}", s1);

    let s2 = String::from("hello");           // s2 进入作用域

    let s3 = takes_and_gives_back(s2);   // s2 被移动到 takes_and_gives_back 中, 它也将返回值移给 s3

    //println!("s2={}", s2); //s2已经move给了s3，所以s2变量无效
    println!("s3={}", s3);


} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {                        // gives_ownership 将返回值移动给调用它的函数

    let some_string = String::from("hello");  // some_string 进入作用域.

    some_string                                         // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}


/**
    在每一个函数中都获取所有权并接着返回所有权有些啰嗦

    多返回值：使用元组
*/
fn test_touple() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}


/**
    借用:
    我们将获取引用作为函数参数称为 借用（borrowing）
*/
fn test_borrowing() {
    let s = String::from("hello");

    change(&s);
}

/**
    借用
*/
fn change(some_string: &String) {
    //some_string.push_str(", world");//尝试修改借用的值, 正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。
}


/**
    可变引用

*/
fn test_mutable_borrowing() {
    //改为mut型的变量s
    let mut s = String::from("hello");

    //创建一个可变引用 &mut s
    change2(&mut s);
}

//接受一个可变引用 some_string: &mut String。
fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}


/**
    可变引用有一个很大的限制：在特定作用域中的特定数据只能有一个可变引用。

    多个可变引用， 错误写法

 */
fn test_multi_mutable_borrowing() {
    let mut s = String::from("hello");

/*    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);*/
}

/**
    多个可变引用，正确写法

 */
fn test_multi_mutable_borrowing2() {

    let mut s = String::from("hello");

    {
        let r1 = &mut s;

    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;

    println!("{}", r2);
}


fn test_find_first_word() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word 的值为 5

    s.clear(); // 这清空了字符串，使其等于 ""

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！

    println!("word={}", word);
}

/**
    编写一个函数，该函数接收一个字符串，并返回在该字符串中找到的第一个单词。
    如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。

*/
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn test_find_first_word_use_slice() {
    let my_string = String::from("hello world");

    // first_word 中传入 `String` 的 slice
    let word = first_word2(&my_string[..]);
    println!("word={}" , word);

    let my_string_literal = "hello world";

    // first_word 中传入字符串字面值的 slice
    let word = first_word2(&my_string_literal[..]);
    println!("word={}" , word);

    // 因为字符串字面值 就是 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word2(my_string_literal);
    println!("word={}" , word);
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn test_slice() {
    //String literals are string slices:
    let hello = "Hello, world!";

    // with an explicit type annotation
    let hello: &'static str = "Hello, world!";
}

