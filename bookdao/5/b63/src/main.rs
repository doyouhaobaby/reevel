fn capitalize(v: &mut [char]) {

}

fn foo() {
    let mut data = vec!['a', 'b', 'c'];
    let slice = &mut data[..];
    capitalize(slice);
}

fn main() {
    println!("Hello, world!");
}
