fn main() {
    let name = "el".to_string();
    let name1 = name.clone();
    let t1 = std::thread::spawn(move || {
        println!("1{}", name);
    });
    let t2 = std::thread::spawn(move || {
        println!("2{}", name1);
    });
    println!("3 {:?}, {:?}", t1.join(), t2.join());
}


// fn main() {
//     let name = "hello".to_string();
//     let t = std::thread::spawn(move || {
//         println!("hello,{}", name);
//     });
//     println!("wait {:?}", t.join());
// }

// fn main() {
//     let mut num = 5;
//     {
//         let mut add_num = move |x: i32| ->i32 {
//             num += x;
//             num
//         };
//         let y = add_num(5);
//         println!("1:{}", num);
//         println!("2:{}", y);
//     }
//     println!("3:{}", num);
// }

// fn main() {
//     let mut num = 5;
//     {
//         let mut add_num = |x: i32| num += x;
//         add_num(5);
//     }
//     println!("{}", num);
// }

// fn main() {
//     let s = String::from("hello");
//     let take_str = || s;
//     println!("{}", s);
//     println!("{}", take_str());
// }

// fn main() {
//     let p = Person{name: "xiaoniuge".to_string(), age: 30};
//     let age = |p: &Person| p.age;
//     let name: for<'a> fn (&'a Person) -> &'a String = |p: &Person| &p.name;
//     println!("{},{}", age(&p), name(&p));
// }

// struct Person{
//     name: String,
//     age: u8,
// }

// fn main() {
//     let plus = |i: i32, m: i32 | i+m;

//     let plus_five = |x| plus(x, 5);


//     println!("{}", plus_five(55));
// }


// fn main() {
//     let mut test = Test {
//         ref_int: &66,
//         ref_str: "xx",
//     };

//     println!("{:?}", test);
//     test.set_int(&99);

//     println!("{:?}", test);

//     test.set_string("xxxxxx");

//     println!("{:?}", test);
// }

// #[derive(Debug)]
// struct Test <'life> {
//     ref_int: &'life i32,
//     ref_str: &'life str,
// }

// impl<'life> Test<'life> {
//     fn set_string(&mut self, s: &'life str) {
//         self.ref_str = s;
//     }
//     fn set_int(&mut self, i: &'life i32) {
//         self.ref_int = i;
//     }
// }

// fn main() {
//     let str1 = String::from("longlonglong");
//     let str2 = "short string";
    
//     let (long_str, short_str) = order_string(str1.as_str(), str2);
//     println!("{}, {}", long_str, short_str);
// }

// fn order_string<'c>(s1: &'c str, s2: &'c str) -> (&'c str, &'c str) {
//     if s1.len() < s2.len() {
//         return (s1, s2);
//     }

//     return (s2, s1);
// }

// fn main() {
//     let r;

//     {
//         let x:i32 = 10;
//         r = &x;
//     }

//     println!("{}", r);
// }

// use std::mem::replace;

// fn main() {
//     let mut x = Render {
//         current_buffer: Buffer {buffer: String::from("xxxx")},
//         next_buffer: Buffer {buffer: String::from("yyy")},
//     };

//     println!("{:?}", x);

//     x.update_buffer(String::from("new buffer"));
//     println!("{:?}", x);
// }

// #[derive(Debug)]
// struct Buffer {
//     buffer: String,
// }

// #[derive(Debug)]
// struct Render {
//     current_buffer: Buffer,
//     next_buffer: Buffer,
// }

// impl Render {
//     fn update_buffer(&mut self, buf: String) {
//         // self.current_buffer = self.next_buffer;
//         // self.next_buffer = Buffer {buffer: buf};
//         self.current_buffer = replace(&mut self.next_buffer, Buffer {buffer: buf});
//     }
// }

//fn main() {
    //println!("Hello, world!");
    // let x = 5;
    // x += 10;

    // let mut x = 5;
    // x += 10;
    // println!("x is {}", x);

    // let mut x:i32 = 5;
    // x += 10;
    // println!("x is {}", x);

    // const LEN:u32 = 1111;
    // println!("const is {}", LEN);

    // let hello = "hello world".to_string();
    // takes_ownership(hello); // 所有权被转移到函数内部
    // println!("{}", hello);

    // let x = gives_ownership();
    // println!("{}", x);

    // let xxx = String::from("xxxx");
    // let y = takes_and_give_back(xxx);
    // println!("{}", y);
    //println!("{}", xxx); //value borrowed here after move
    // let x = Person {
    //     name: String::from("xiaoniuge"),
    //     age: 30,
    //     email: String::from("xxx@88.com"),
    //     sex: Sex::Male
    // };

    // println!("{:?}", x);

    // let mut x = Person {
    //     name: String::from("xiaoniuge"),
    //     email: String::from("xiaoniuge@88.com")
    // };

    // println!("{:?}", x);

    // let _name = x.name;
    // println!("{}, {}", _name, x.email);
    // //println!("{}", x.name);//^^^^^^ value borrowed here after move
    // //println!("{:?}", x);

    // x.name = String::from("999");
    // println!("x{:?}", x);
//}

// #[derive(Debug)]
// struct Person {
//     name: String,
//     email: String
// }

// #[derive(Debug)]
// enum Sex {
//     Male, Female
// }

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8,
//     email: String,
//     sex: Sex
// }

// fn takes_and_give_back(mut x: String) -> String {
//     x.push_str("xxx");
//     x
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn gives_ownership() -> String {
//     let hello = String::from("xxxx");
//     hello
// }
