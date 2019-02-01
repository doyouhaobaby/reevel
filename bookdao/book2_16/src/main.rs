fn main() {
    let n =  8;
    let big_n = if (n < 10 && n > -10) {
       11 * n    
    } else {
       n / 2 
    };

    //assert_eq!(big_n, 6);// n = 13 ,big_n = 6
    assert_eq!(big_n, 88);
}
