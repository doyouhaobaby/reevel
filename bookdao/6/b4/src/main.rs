fn f() {
    println!("3");
}

fn main() {
    f();

    {
        fn f() {
            println!("xxx");
        }
        f();

        {
            fn f() {
                println!("haha");
            }
            f();
        }
    }
}
