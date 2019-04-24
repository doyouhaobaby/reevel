fn call<F: FnOnce()>(f: F) {
    f()
}

fn main() {
    let mut x = 2;
    let incre_x = || x += 5;
    call(incre_x);
    //call(incre_x);//value moved here
    //call(incre_x);//value moved here

    println!("{}", x);

    // ---- split

    let mut x = 2;
    let incre_x = move || x += 5;
    call(incre_x);
    call(incre_x);
    call(incre_x);
    println!("{}", x);

    // ---- split
    let mut x = vec![1, 2];
    let t = move || x.push(5);
    call(t);
    // call(t);//^ value used here after move
}
