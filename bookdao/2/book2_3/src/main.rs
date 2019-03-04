fn main() {
    pub fn temp() -> i32 {
        1
    }

    let a = &temp();
    temp() = *a;
}
