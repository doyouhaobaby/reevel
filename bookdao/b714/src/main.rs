use std::fmt;
use std::convert::From;
use std::str::FromStr;

fn main() {
   let hi  = "hello".red().on_yellow();
   println!("{:?}", hi);

   println!("{}", hi);

   let hi = "hello world".red();
   println!("{}", hi);

   let hi = "hello world".on_yellow();
   println!("{}", hi);
   
   let hi = "blue".on_blue();
   println!("{}", hi);

   let hi = "blue red".on_blue().red();
   println!("{}", hi);

//    let hi = "test red".color("red").color("red");
//    println!("{}", hi); 

   let hi = "test red".color("red");
   println!("{}", hi); 

   let hi = "test red".color("rEd");
   println!("{}", hi); 

   let hi = "test red".color("blue".to_string());
   println!("{}", hi);
}

//#[derive(Debug,Clone,Eq,PartialEq)]
#[derive(Debug)]
struct ColoredString {
    input: String,
    fgcolor: Option<Color>,
    bgcolor: Option<Color>,
}

trait Colorize {
    fn red(self: Self) -> ColoredString;
    fn yellow(self: Self) -> ColoredString;
    fn blue(self: Self) -> ColoredString;
    fn color<S: Into<Color>>(self: Self, color: S) -> ColoredString;
    fn on_color<S: Into<Color>>(self: Self, color: S) -> ColoredString;
    fn on_red(self) -> ColoredString;
    fn on_blue(self) -> ColoredString;
    fn on_yellow(self) -> ColoredString;
}

impl Default for ColoredString {
    fn default() -> Self {
        ColoredString {
            input: String::default(),
            fgcolor: None,
            bgcolor: None,
        }
    }
}

impl Colorize for ColoredString {
    fn red(self: Self) -> ColoredString {
        self.color(Color::Red)
    }
    fn blue(self: Self) -> ColoredString {
        self.color(Color::Blue)
    }
    fn yellow(self: Self) -> ColoredString {
        self.color(Color::Yellow)
    }
    fn color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            fgcolor: Some(color.into()), ..self
        } 
    }
    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            bgcolor: Some(color.into()), ..self
        }
    }
    fn on_yellow(self: Self) -> ColoredString {
        self.on_color(Color::Yellow)
    }
    fn on_blue(self: Self) -> ColoredString {
        self.on_color(Color::Blue)
    }
    fn on_red(self: Self) -> ColoredString {
        self.on_color(Color::Red)
    }
}

impl Colorize for &'static str {
    fn red(self: Self) -> ColoredString {
        self.color(Color::Red)
    }
    fn blue(self: Self) -> ColoredString {
        self.color(Color::Blue)
    }
    fn yellow(self: Self) -> ColoredString {
        self.color(Color::Yellow)
    }
    fn color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            input: String::from(self),
            fgcolor: Some(color.into()), ..ColoredString::default()
        }
    }
    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            input: String::from(self),
            fgcolor: Some(color.into()), ..ColoredString::default()
        }
    }
    fn on_yellow(self: Self) -> ColoredString {
        self.on_color(Color::Yellow)
    }
    fn on_blue(self: Self) -> ColoredString {
        self.on_color(Color::Blue)
    }
    fn on_red(self: Self) -> ColoredString {
        self.on_color(Color::Red)
    }
}

impl ColoredString {
    fn compute_style(&self) -> String {
        let mut res = String::from("\x1B[");
        let mut has_wrote = false;

        if let Some(bgcolor) = &self.bgcolor {
            res.push_str(bgcolor.to_bg_str());
            has_wrote = true;
        }

        if let Some(fgcolor) = &self.fgcolor {
            if has_wrote { res.push(';') };
            res.push_str(fgcolor.to_fg_str());
        }
        res.push('m');
        res
    }
}

impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("i am here");
        let input = &self.input.clone();
        
        f.write_str(&self.compute_style());
        f.write_str(input);
        f.write_str("\x1B[0m");
        Ok(())
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Yellow,
    Blue,
}

impl Color {
    fn to_fg_str(&self) -> &str {
        match self {
            Color::Red => "31",
            Color::Yellow => "33",
            Color::Blue => "34",
        }
    }
    fn to_bg_str(&self) -> &str {
        match self {
            Color::Red => "31",
            Color::Yellow => "33",
            Color::Blue => "34",
        }
    }
}

impl <'a> From<&'a str> for Color {
    fn from(src: &str) -> Self {
        println!("{:?}", src);
        src.parse().unwrap_or(Color::Red)
    }
}

impl From<String> for Color {
    fn from(src: String) -> Self {
        println!("{:?}", src);
        src.parse().unwrap_or(Color::Yellow)
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        println!("----------------");
        println!("{:?}", src);
        let src = src.to_lowercase();
        println!("{:?}", src);
        //println!("{:?}", src.as_ref());
        println!("----------------");
        match src.as_ref() {
            "red" => Ok(Color:: Red),
            "yellow" => Ok(Color:: Yellow),
            "blue" => Ok(Color:: Blue),
            _ => Err(()),
        }
    }
}
