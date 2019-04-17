#[derive(Debug)]
struct User{
    name: &'static str,
    avatar: &'static str,
}

impl User {
    fn show(&self) {
        println!("name {}\n", self.name);
        println!("avatar {}\n", self.avatar);
    }
}

fn main() {
    let u = User {
        name: "小牛哥",
        avatar: "http://queryphp.com/a.jpg",
    };

    println!("{:?}", u);
    u.show();
    User::show(&u);
}
