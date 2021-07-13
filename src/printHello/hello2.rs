
//对于目录来说，要想成为一个模块，需要在目录下创建mod.rs，并在mod.rs中声明

pub fn print_hello2() {
    println!("hello2!");

    //同一级定义的模块可以直接使用
    sub_mod::sub_mod_hello();
    sub_mod::sub_mod2::sub_mod2_print_hello()
}


mod sub_mod {

    pub(crate) fn sub_mod_hello() {
        println!("sub_mod_hello(), hello!")
    }

    pub mod sub_mod2 {
        pub fn sub_mod2_print_hello() {
            println!("sub_mod2_print_hello(), hello!")
        }
    }

}