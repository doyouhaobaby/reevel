use std::fmt;

fn main() {
   let hi  = "hello".red().on_yellow();
   println!("{:?}", hi);

   println!("{}", hi);

   let hi = "hello world".red();
   println!("{}", hi);

   let hi = "hello world".on_yellow();
   println!("{}", hi);
}

#[derive(Debug)]
struct ColoredString {
    input: String,
    fgcolor: String,
    bgcolor: String,
}

trait Colorize {
    const FG_RED : &'static str = "31";
    const BG_YELLOW : &'static str = "43";
    fn red(self: Self) -> ColoredString;
    fn on_yellow(self) -> ColoredString;
}

impl Default for ColoredString {
    fn default() -> Self {
        ColoredString {
            input: String::default(),
            fgcolor: String::default(),
            bgcolor: String::default(),
        }
    }
}

impl Colorize for ColoredString {
    fn red(self: Self) -> ColoredString {
        ColoredString {
            fgcolor: String::from(ColoredString::FG_RED), ..self
        }
    }
    fn on_yellow(self) -> ColoredString {
        ColoredString {
            bgcolor: String::from(ColoredString::BG_YELLOW), ..self
        }
    }
}

impl Colorize for &'static str {
    fn red(self: Self) -> ColoredString {
        ColoredString {
            input: String::from(self),
            fgcolor: String::from(ColoredString::FG_RED), ..ColoredString::default()
        }
    }
    fn on_yellow(self) -> ColoredString {
        ColoredString {
            input: String::from(self),
            bgcolor: String::from(ColoredString::BG_YELLOW), ..ColoredString::default()
        }
    }
}

impl ColoredString {
    fn compute_style(&self) -> String {
        let mut res = String::from("\x1B[");
        let mut has_wrote = false;

        if !self.bgcolor.is_empty() {
            res.push_str(&self.bgcolor);
            has_wrote = true;
        }

        if !self.fgcolor.is_empty() {
            if has_wrote { res.push(';') };
            res.push_str(&self.fgcolor);
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
