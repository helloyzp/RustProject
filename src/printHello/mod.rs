//对于目录来说，要想成为一个模块，需要在目录下创建mod.rs，并在mod.rs中声明当前的目录包含哪些子模块
//printHello这个目录包含hello2.rs和hello2.rs两个模块，因此要在mod.rs中声明hello2和hello3
//通过mod.rs从而将printHello目录变为printHello模块，供其他模块引用

pub(crate) mod hello2; //默认是私有的，只能在当前目录下访问, 要想被其他目录下的模块访问，需要加上pub修饰符
mod hello3;