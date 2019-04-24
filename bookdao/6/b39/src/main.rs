#![feature(fn_traits)]

fn main() {
    let mut s = "hello".to_string();

    {
        let mut c = || s += " rust";
        c();
        //this closure implements `FnMut`, not `Fn`
        //c.call(());
        c.call_once(());
        // value used here after move
       // c.call_once(());
       println!("{:?}", s);
    }

    println!("{:?}", s);
}
