fn main() {
    //let n = 1;
    //let n = 0;
    //let n = 2;
    //let n = 22;
    let n = 55;
    //let n = 100;

    match n {
        0 => println!("0 is print"),//0
        1...3 => println!("1 - 3"),//1,2
        | 8 | 9 | 22 => println!("special number"),//22
        m @ 55 => println!("{} will found", m),//55
        _ => println!("common"),//100
    }
}
