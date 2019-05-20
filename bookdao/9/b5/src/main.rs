fn get_shortest(vec: Vec<&str>) -> Option<&str> {
    if vec.len() > 0 {
        let mut short = vec[0];
        for name in vec.iter() {
            if name.len() < short.len() {
                //short = name;// &str 复制语义也可以
                short = *name;
            }
        }
        Some(short)
    } else {
        None
    }
}

fn show_shortest(vec: Vec<&str>) -> &str {
    match get_shortest(vec) {
        Some(v) => v, 
        None => "Not Found",
    }
}

fn main() {
    assert_eq!(show_shortest(vec!["Uku", "Felisss"]), "Uku");
    assert_eq!(show_shortest(Vec::new()), "Not Found");
}
