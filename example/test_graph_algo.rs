fn main() {
    let mut ind = Vec::new();
    let lens = 3;
    let mut i = 1;
    while i < lens-1 {
        ind.push(0);
        ind.push(i);
        i=i+1;
        ind.push(i);
    }
    println!("hello{:?}", ind);
}