use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

/*
<?php 
interface IFoo {
    const ERR = 'ParseIntError';
}

class Bar implement IFoo{
    public function from_str($str) {
        echo self::ERR;
    }
}
?>
*/

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hello = s
            .trim_matches(|p| 
                p == '{' || p == '}' 
            )
            .split(",")
            .collect::<Vec<&str>>();

        //dbg! (hello);
        // dbg! (hello);
        //|         ------------- value moved here
        dbg! (hello.clone());

        let x_v = hello[0].parse::<i32>()?;
        let y_v = hello[1].parse::<i32>()?; 

        Ok(Point {x: x_v, y: y_v})
    }
}

fn main() {
     let p = Point::from_str("{55,56}");
     assert_eq!(Point{x: 55, y: 56}, p.unwrap());

     let _e = Point::from_str("{55,xxx}");
     //Err(ParseIntError { kind: InvalidDigit })
     //println!("{:?}", e);
}
