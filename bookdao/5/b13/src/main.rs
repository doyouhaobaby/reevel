fn main() {
    let a = 1;
    let b = "hello world".to_string();

    {
        let c = a;
        let d = b;
    }

    println!("{:?}", a);
   // println!("{:?}", b);//^ value borrowed here after move
}
