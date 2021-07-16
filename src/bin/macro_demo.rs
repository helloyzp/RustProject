use std::result::Result;

/*#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    Ok(())
}*/


//定义宏
macro_rules! print_hello {
    ()=> {
     println!("hello world！");
    }
}


#[test]
fn test_print_hello() {
    //使用宏
    print_hello!();
}

fn main() {

}

