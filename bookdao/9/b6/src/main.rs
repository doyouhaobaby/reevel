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
    //get_shortest(vec).unwrap()// 报错，未处理 not found 问题
    //get_shortest(vec).unwrap_or("Not Found")
    get_shortest(vec).unwrap_or_else(|| "Not Found")
   // get_shortest(vec).expect("Not Found")// 报错，直接线程恐慌
}

fn main() {
    assert_eq!(show_shortest(vec!["Uku", "Felisss"]), "Uku");
    assert_eq!(show_shortest(Vec::new()), "Not Found");
}
