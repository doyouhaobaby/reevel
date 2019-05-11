fn main() {
    let arr = [1, 2, 3];
    let result1 = arr.iter().any(|&x| x != 2);
    assert!(result1);
    //let result1 = arr.iter().any(|x| x != 2);//^^ no implementation for `&{integer} == {integer}`
}
