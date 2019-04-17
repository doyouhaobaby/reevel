type MathOp = fn(i32, i32) -> i32; 

fn math(m: &'static str) -> MathOp {
    fn sum(a: i32, b: i32) -> i32 {
       a + b 
    }

    fn product(a: i32, b: i32) -> i32 {
       a * b 
    }

    match m {
        "sum" => sum,
        "product" => product,
        _ => {
            println!("not found op {},replace with sum", m);
            sum
        },
    }
}

fn main() {
    let (a, b) = (3, 5);
    let sum = math("sum");
    let product = math("product");
    let div = math("div");
    assert_eq!(sum(a, b), 8);
    assert_eq!(product(a, b), 15);
    assert_eq!(div(a, b), 8);
}
