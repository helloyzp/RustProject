
fn main() {
    //一、所有权的移动(move)
    let s1 = String::from("hello"); //s1拥有字符串"hello"的所有权
    let s2 = s1; //发生了所有权的移动，此时s2拥有字符串"hello"的所有权，并且变量s1将不可以再使用

    //println!("s1={}", s1);  //变量s1将不可以再使用
    println!("s2={}", s2);


    //二、所有权的借用(borrow)
    let s3 = String::from("hello borrow");
    print_hello(&s3); //
    println!("s3={}", s3); //


    //三、所有权的克隆(clone)，只拷贝值给新的变量，但是值的所有权并没有给新的变量，值的所有权并没有发生转移
    let s4 = String::from("hello world");
    let s5 = s4.clone(); //此时有两个值都是"hello world"，只是第一个"hello world"的所有者是s4，第二个"hello world"的所有者是s5，
    println!("s4={}", s4);//s4和s5是两个不同的字符串变量，只是它们的值都是"hello world"
    println!("s5={}", s5);



    //四、所有权的复制(copy)
    let s6 = 1; //整数类型实现了copy这个trait(接口)，因此在赋值操作时自动会调用clone(), 因此 let s7 = s6; 就相当于 let s7 = s6.clone();
    let s7 = s6;
    println!("s6={}", s6); //此时s6和s7指向的是两个不同的1
    println!("s7={}", s7);

}


/**
 我们将获取引用作为函数参数称为 借用（borrowing）

 参数类型是&String，是字符串的借用，借用的语法就是&符号
 &String就表示String类型的借用
 */
fn print_hello(s: &String) {
    println!("value={}", s);
}

/*trait Copy {
//实现了Copy这个trait的类，在赋值时会自动clone()，不需要手动调用clone()

}*/