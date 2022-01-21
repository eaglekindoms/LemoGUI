use std::fmt::Debug;

pub fn indices() {
    let mut ind = Vec::new();
    let lens = 3;
    let mut i = 1;
    while i < lens - 1 {
        ind.push(0);
        ind.push(i);
        i = i + 1;
        ind.push(i);
    }
    println!("hello{:?}", ind);
}

pub fn num<T>(x: T) -> T
where
    T: Debug,
{
    println!("{:?}", x);
    x
}

pub fn closure<F>(num: F)
where
    F: Fn(i32) -> i32,
{
    let x = num(12);
    println!("{:?}", x);
}

pub fn closure1<T>(x: T, num: &dyn Fn(T) -> T) {
    num(x);
}

pub fn closure2<T>(x: T, num: Box<dyn Fn(T) -> T>) {
    num(x);
}

pub fn closure3() -> Box<dyn Fn(i32) -> i32> {
    let num = 12;
    Box::new(move |x| x + num)
}
