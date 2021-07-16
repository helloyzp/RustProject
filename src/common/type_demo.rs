

// 允许以非驼峰的方式起别名，否则起unsigned_64别名时会提示： Type `unsigned_64` should have a camel case name such as `Unsigned64`
#![allow(non_camel_case_types)]

//为u64起别名
type unsigned_64 = u64;



#[test]
fn test_type() {
    let i :unsigned_64 = 1;
    println!("i={}" , i);
}


use std::result;
enum ConcreteError {
    Foo,
    Bar,
}
type Result<T> = result::Result<T, ConcreteError>;



fn main() {


}


