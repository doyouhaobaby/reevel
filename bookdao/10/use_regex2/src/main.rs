use regex::Regex;

fn main() {
    let re = Regex::new(r"(?x)
        (?P<year>\d{4}) # the year
        -
        (?P<month>\d{2}) # 月
        -
        (?P<day>\d{2}) # 天
    ").unwrap();

    let caps = re.captures("2019-03-24").unwrap();

    assert_eq!("2019", &caps["year"]);
    assert_eq!("03", &caps["month"]);
    assert_eq!("24", &caps["day"]);

    let after = re.replace_all("2019-03-24", "$month/$day/$year");
    assert_eq!(after, "03/24/2019");

    let after = re.replace_all("2019-03-24", "$month/$notfoundday/$year");
    assert_eq!(after, "03//2019");
}
