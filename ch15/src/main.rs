use std::{fmt::Display, ops::Deref};

use List::{Cons, Nil};

fn main() {
    let box_str = Box::new("box of str");
    print_str(&box_str);

    let list = Cons(
        1,
        Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Nil))))))),
    );

    println!("{:?}", list);

    let my_box = MyBox::new(10);
    println!("{}", *my_box);

    let mb2 = MyBox::new("hi");
    print_str(*mb2);

    let mb3 = MyBox::new(String::from("Rust"));
    print_str(&*mb3);

    // # Deref 强制转换如何与可变性交互
    // 类似于如何使用 Deref trait 重载不可变引用的 * 运算符，Rust 提供了 DerefMut trait 用于重载可变引用的 * 运算符。

    // Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换：

    // 当 T: Deref<Target=U> 时从 &T 到 &U。
    // 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
    // 当 T: Deref<Target=U> 时从 &mut T 到 &U。
    // 头两个情况除了可变性之外是相同的：第一种情况表明如果有一个 &T，而 T 实现了返回 U 类型的 Deref，则可以直接得到 &U。第二种情况表明对于可变引用也有着相同的行为。

    // 第三个情况有些微妙：Rust 也会将可变引用强转为不可变引用。但是反之是 不可能 的：不可变引用永远也不能强转为可变引用。因为根据借用规则，如果有一个可变引用，其必须是这些数据的唯一引用（否则程序将无法编译）。将一个可变引用转换为不可变引用永远也不会打破借用规则。将不可变引用转换为可变引用则需要初始的不可变引用是数据唯一的不可变引用，而借用规则无法保证这一点。因此，Rust 无法假设将不可变引用转换为可变引用是可能的。

    let c = CoustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CoustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
}

struct CoustomSmartPointer {
    data: String,
}

impl Drop for CoustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn print_str<T>(item: T)
where
    T: Copy + Clone + Display,
{
    println!("{}", item);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    //貌似这样写也可以，还不错
    // fn new(x:T)->Self{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
