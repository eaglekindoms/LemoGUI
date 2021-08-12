use crate::test_closure::*;

mod test_closure;
mod test_alter_message;

fn main() {
    closure(|x| x + 2);
    closure1::<i32>(12, &num);
    closure2::<f32>(12., Box::new(num));
    let x = closure3()(2);
    println!("{:#?}", x);
}