mod a {
    fn x() -> u8 {
        5
    }

    pub mod example {
        //use super::x;
        use crate::a::x;

        pub fn foo() {
            println!("{}", x());
        }
    }
}

fn main()
{
    a::example::foo();
}