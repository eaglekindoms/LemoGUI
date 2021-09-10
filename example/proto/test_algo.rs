use cgmath::{Matrix4, Vector4};

use crate::test_closure::*;

mod test_closure;
mod test_alter_message;

fn main() {
    closure(|x| x + 2);
    closure1::<i32>(12, &num);
    closure2::<f32>(12., Box::new(num));
    let x = closure3()(2);
    println!("{:#?}", x);

    let width: f32 = 400.;
    let height: f32 = 800.;
    let projection: Matrix4<f32> = orthographic_projection(width, height).into();
    let position: Vector4<f32> = Vector4::new(110.5, 110.5, 0.0, 0.0);
    println!("{:?}", projection);
    let p = projection * position;
    println!("{:?}", position);
    println!("{:?}", p);
}

fn orthographic_projection(w: f32, h: f32) -> [[f32; 4]; 4] {
    [
        [2.0 / w, 0.0, 0.0, 0.0],
        [0.0, 2.0 / h, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-1.0, -1.0, 0.0, 1.0],
    ]
}