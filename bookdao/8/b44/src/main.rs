fn main() {
    let s = [0, 1, 1, 23, 33, 2, 33, 33, 44, 99, 3];
    assert_eq!(s.binary_search(&44), Ok(8));
    assert_eq!(s.binary_search(&221212), Err(11));
    assert_eq!(s.binary_search(&1), Ok(2));

    let r = s.binary_search(&33);
    dbg!(r.clone());
    assert!(match r {
        Ok(3) => true,
        Ok(4) => true,
        Ok(6) => true,
        Ok(7) => true,
        _ => false,
    });

    let seek = 44;
    assert_eq!(
        s.binary_search_by(|probe| probe.cmp(&seek)),
        Ok(8),
    );
    let s = [(0,0), (98,2), (4,8),(5,34)];
    assert_eq!(
        s.binary_search_by_key(&4, |&(a,b)| {
            println!("------1111");
            dbg!(a);
            dbg!(b);
            a
        }),
        Ok(2),
    );
    assert_eq!(
        s.binary_search_by_key(&4, |&(a,b)| {
            println!("------22222");
            dbg!(a);
            dbg!(b);
            b
        }),
        Err(2),
    );
    assert_eq!(
        s.binary_search_by_key(&34, |&(a,b)| {
            println!("------333333");
            dbg!(a);
            dbg!(b);
            b
        }),
        Ok(3),
    );
}
