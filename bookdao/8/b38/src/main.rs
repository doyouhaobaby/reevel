use std::fmt::{self, Formatter, Display};
struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let a = if self.lat >= 0.0 { 'N' } else { 'S'};
        let b = if self.lon >= 0.0 {'E'} else {'W'};
        write! (f, "name {}, lat {:.3} {}, lon {:.3} {}", self.name, self.lat, a, self.lon, b)
    }
}

fn main() {
    let city = City {name: "北京", lat: 39.90469, lon: -116.40717};
    assert_eq!(format!("{}", city), "name 北京, lat 39.905 N, lon -116.407 W");
    println!("{}", city);
}
