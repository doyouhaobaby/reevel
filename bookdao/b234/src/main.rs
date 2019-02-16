#[derive(Debug, PartialEq)]
struct People {
    name: &'static str,
    gender: u32,
}

impl People {
    fn new (name: &'static str, gender: u32) -> Self {
        return People{name: name, gender: gender};
    }

    fn name (&self) {
        println!("{}", self.name);
    }

    fn set_name (&mut self, name: &'static str) {
        self.name = name;
    }
    
    fn gender (&self) {
        let gender = if (self.gender == 1) { "boy" } else { "girl" };
        println!("{}", gender);
    }
}

fn main() {
   let xng = People::new("xiaoniuge", 1);
    println!("{:?}", xng);
    assert_eq!(xng, People {name: "xiaoniuge", gender: 1});

    xng.name();
    xng.gender();

    let mut xng = People::new("zhuge", 2);
    xng.name();
    xng.gender();

    xng.set_name("我是你哥");
    xng.name();
    xng.gender();

}
