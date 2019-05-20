fn get_short_length(vec: Vec<&str>) -> Option<usize> {
    match get_shortest(vec) {
        Some(v) => Some(v.len()),
        None => None,
    }
}

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

fn main() {
    assert_eq!(get_short_length(vec!["uku", "xxxxxx"]), Some(3));
    assert_eq!(get_short_length(Vec::new()), None);
}
