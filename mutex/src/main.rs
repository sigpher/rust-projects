use std::{
    sync::{Arc, Mutex},
    thread,
};

// 由于不需要语言提供并发相关的基础设施，并发方案不受标准库或语言所限：我们可以编写自己的或使用别人编写的并发功能。
// 然而有两个并发概念是内嵌于语言中的：std::marker 中的 Sync 和 Send trait。
// 通过 Send 允许在线程间转移所有权
// 几乎所有基本类型都是 Send 的，除了第十九章将会讨论的裸指针（raw pointer）。


// 成功了！我们从 0 数到了 10，这可能并不是很显眼，不过一路上我们确实学习了很多关
// 于 Mutex<T> 和线程安全的内容！这个例子中构建的结构可以用于比增加计数更为复杂的操作。
// 使用这个策略，可将计算分成独立的部分，分散到多个线程中，接着使用 Mutex<T> 使用各自
// 的结算结果更新最终的结果。


// Sync 允许多线程访问
// Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用。
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("{}", &num);
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

#[derive(Debug)]
struct User<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> User<'a> {
    fn new(name: &str, age: u8) -> User {
        User { name, age }
    }
}
