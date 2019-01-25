fn main() {
    pub fn answer() -> () {
       let a = 5;
       let b = 10;

       assert_eq!(sum(a, b), 15); 
    }

    pub fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    answer();
}
