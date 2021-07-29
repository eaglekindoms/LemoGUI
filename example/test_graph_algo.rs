use std::fmt::Debug;

fn main() {
    closure(|x| x + 2);
    closure1::<i32>(12, &num);
    closure2::<f32>(12., Box::new(num));
    let x = closure3()(2);
    println!("{:#?}", x);
}

fn indices() {
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

fn num<T>(x: T) -> T
    where T: Debug
{
    println!("{:?}", x);
    x
}

fn closure<F>(num: F)
    where F: Fn(i32) -> i32 {
    let x = num(12);
    println!("{:?}", x);
}

fn closure1<T>(x: T, num: &Fn(T) -> T) {
    num(x);
}

fn closure2<T>(x: T, num: Box<dyn Fn(T) -> T>)
{
    num(x);
}

fn closure3() -> Box<dyn Fn(i32) -> i32>
{
    let num = 12;
    Box::new(move |x| x + num)
}
