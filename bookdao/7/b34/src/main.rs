#[derive(Debug,Clone)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

#[derive(Debug,Clone)]
struct CircleBuild {
    x: f64,
    y: f64,
    radius: f64,   
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    fn new() -> CircleBuild {
        CircleBuild {x: 0.0, y: 0.0, radius: 1.0,}
    }
}

impl CircleBuild {
    fn x(&mut self, v: f64) -> &mut CircleBuild {
        self.x = v;
        self
    }
    fn y(&mut self, v: f64) -> &mut CircleBuild {
        self.y = v;
        self
    }
    fn radius(&mut self, v: f64) -> &mut CircleBuild {
        self.radius = v;
        self
    }
    fn build(&self) -> Circle {
        Circle {x: self.x, y: self.y, radius: self.radius,}
    }
}

fn main() {
    let c = Circle::new()
        .x(1.0)
        .y(2.0)
        .radius(5.0)
        .build();
    
    dbg!(c.clone());

    assert_eq!(c.area(), 78.53981633974483);
    assert_eq!(c.x, 1.0);
    assert_eq!(c.y, 2.0);
}
