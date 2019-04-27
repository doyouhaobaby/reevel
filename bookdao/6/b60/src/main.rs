// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

struct Counter {
    count: usize,
}

impl Iterator for Counter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        } 
    }
}

fn main() {
    let mut c = Counter {
        count: 0
    };

    assert_eq!(Some(1), c.next());
    assert_eq!(Some(2), c.next());
    assert_eq!(Some(3), c.next());
    assert_eq!(Some(4), c.next());
    assert_eq!(Some(5), c.next());
    assert_eq!(None, c.next());
}
