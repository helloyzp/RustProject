
fn main() {


}

fn function1() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

//我们可以创建一层抽象，在这个例子中将表现为一个获取任意整型列表作为参数并对其进行处理的函数。
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*
//在函数定义中使用泛型
//类型参数声明位于函数名称与参数列表中间的尖括号 `<>` 中
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/


// 结构体定义中的泛型
struct Point<T> {
    x: T,
    y: T,
}

#[test]
fn test_struct_generic() {
    //字段 x 和 y 的类型必须相同，因为他们都有相同的泛型类型 T
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

mod struct_generic{

    //如果想要定义一个 x 和 y 可以有不同类型且仍然是泛型的 Point 结构体，我们可以使用多个泛型类型参数。
    struct Point<T, U> {
        x: T,
        y: U,
    }

    #[test]
    fn test_struct_generic() {
        let both_integer = Point { x: 5, y: 10 };
        let both_float = Point { x: 1.0, y: 4.0 };
        let integer_and_float = Point { x: 5, y: 4.0 };
    }
}


//枚举定义中的泛型
enum Result<T, E> {
    Ok(T),
    Err(E),
}



//方法定义中的泛型
mod function_generic{

    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    fn main() {
        let p = Point { x: 5, y: 10 };

        println!("p.x = {}", p.x());
    }

}




//定义 Summary 这个 trait
pub trait Summary {
    fn summarize(&self) -> String;
}

//定义 NewsArticle 结构体
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//在 NewsArticle 结构体上实现Summary
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

//定义 Tweet 结构体
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//在 Tweet 结构体上实现Summary
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


fn test() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

//trait的默认实现
mod default_Implementations {

    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    // 指定一个空的 impl 块
    impl Summary for NewsArticle {

    }

}



//trait 作为参数
//参数item是实现了 Summary trait 的某种类型
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}


//返回实现了 trait 的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}