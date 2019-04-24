fn main() {
    let mut s = "hello".to_string();
     {
        let mut c = || s += " rust";
        c();
        c();
        c();
        println!("inner {:?}", s);
     }

     println!("{:?}", s);
}
