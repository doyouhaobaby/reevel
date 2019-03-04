extern "C" {
    fn isalnum(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("tet {}", isalnum(3));
    }
}
