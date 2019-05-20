fn get_short_length(vec: Vec<&str>) -> Option<usize> {
   get_shortest(vec).map(|name| name.len())
}

fn get_short_length2(vec: Vec<&str>) -> usize {
   get_shortest(vec).map_or(0, |name| name.len())
}

fn get_short_length3(vec: Vec<&str>) -> usize {
   get_shortest(vec).map_or_else(|| 44, |name| name.len())
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
    assert_eq!(get_short_length2(vec!["uku", "xxxxxx"]), 3);
    assert_eq!(get_short_length2(Vec::new()), 0);
    assert_eq!(get_short_length3(vec!["uku", "xxxxxx"]), 3);
    assert_eq!(get_short_length3(Vec::new()), 44);
}
