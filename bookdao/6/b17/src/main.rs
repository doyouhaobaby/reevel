type MathOp = fn(i32, i32) -> i32; 

fn sum(a: i32, b: i32) -> i32 {
    a + b 
}

fn product(a: i32, b: i32) -> i32 {
    a * b 
}

fn math(m: &'static str, a:i32, b:i32) -> MathOp {
    match m {
        "sum" => sum(a, b),
                // ^^^^^^^^^ expected fn pointer, found i32
        "product" => product(a, b),
        _ => {
            println!("not found op {},replace with sum", m);
            sum
        },
    }
}

fn main() {
    let (a, b) = (3, 5);
    //let sum = math("sum", a, b);
}
