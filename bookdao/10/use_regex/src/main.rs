use regex::Regex;

const TO_SEARCH: &'static str = "On 2019-03-24 happy.On 2020-01-01 新年";

fn main() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    for caps in re.captures_iter(TO_SEARCH) {
        println!("y {}, m {}, d{}", caps.get(1).unwrap().as_str(), 
            caps.get(2).unwrap().as_str(), caps.get(3).unwrap().as_str());

        //dbg!( caps.get(5).unwrap() );
    }
}
