fn main() {
    let v = "acdsdsfdsfxxxdsfsfsdfsdf";
    let s = v.matches("f").collect::<Vec<&str>>();
    assert_eq!(s, ["f", "f", "f", "f", "f", "f"]);

    let v = "axx1xdfs4xxxxxss3ssssss";
    let s = v.rmatches(char::is_numeric).collect::<Vec<&str>>();
    assert_eq!(s, ["3", "4", "1"]);

    let v = "xxxabcyyyabczzzzabc";
    let s = v.match_indices("abc").collect::<Vec<_>>();
    assert_eq!(s, [(3, "abc"), (9, "abc"), (16, "abc")]);

    let v = "xxxabcyyyabczzzzabc";
    let s = v.match_indices("abc").collect::<Vec<(usize, &str)>>();
    assert_eq!(s, [(3, "abc"), (9, "abc"), (16, "abc")]);

    let v = "xxxabcyyyabczzzzabc";
    let s = v.rmatch_indices("abc").collect::<Vec<(usize, &str)>>();
    assert_eq!(s, [(16, "abc"), (9, "abc"), (3, "abc")]);
}
