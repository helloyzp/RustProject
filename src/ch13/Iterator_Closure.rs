

pub trait Iterator {
    //定义了 trait 的 关联类型（associated type）
    //这段代码表明实现 Iterator trait 时需要要同时定义一个 Item 的具体类型
    //这里 Item 类型被用作 next 方法的返回值类型。换句话说，Item 类型将是迭代器返回元素的类型。
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 此处省略了方法的默认实现
}


struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

//接下来将为 Counter 类型实现 Iterator trait
impl Iterator for Counter {

    type Item = u32; //这里将迭代器的关联类型 Item 设置为 u32，意味着迭代器会返回 u32 值集合。

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}


///展示了一个测试用来演示使用 Counter 结构体的迭代器功能
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    println!("all test passed!")

}