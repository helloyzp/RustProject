

// 允许以非驼峰的方式起别名，否则起unsigned_64别名时会提示： Type `unsigned_64` should have a camel case name such as `Unsigned64`
#![allow(non_camel_case_types)]

//为u64起别名, 非驼峰的方式
type unsigned_64 = u64;

//为u64起别名, 驼峰的方式
type Unsigned64 = u64;


#[test]
fn test_type() {
    let i :unsigned_64 = 1;
    println!("i={}" , i);
}



//还可以使用泛型类型别名
mod generic_alias{
    use std::result;
    enum ConcreteError {
        Foo,
        Bar,
    }

    //还可以使用泛型类型别名：
    type Result<T> = result::Result<T, ConcreteError>;
}



/// Balance of an account.
pub type Balance = u128;


#[test]
fn test_balance() {
    type Balance = super::type_demo::Balance;
    let value:Balance = 500;

    println!("value={}", value);


}


pub type Weight = u64;

///定义一个名称为WeightInfo的trait
pub trait WeightInfo {
    fn transfer() -> Weight;
}

pub trait Config {
    type WeightInfo : WeightInfo ;

}

