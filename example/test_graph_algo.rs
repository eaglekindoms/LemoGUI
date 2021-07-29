use std::fmt::Debug;

fn main() {
    closure::<i32>(12,&num);
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

fn closure<T>(x:T,num: &Fn(T)->T){
    num(x);
}
