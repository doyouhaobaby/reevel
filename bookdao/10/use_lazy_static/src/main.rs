use regex::Regex;
#[macro_use] extern crate lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?x)
        (?P<year>\d{4}) # the year
        -
        (?P<month>\d{2}) # 月
        -
        (?P<day>\d{2}) # 天
    ").unwrap();

    static ref EMAIL_RE: Regex = Regex::new(r"(?x)
        ^\w+@(?:gmail|163|126|qq)\.(?:com|cn|com\.cn|net|org)$
    ").unwrap();
}

fn regex_date(text: &str) -> regex::Captures {
    RE.captures(text).unwrap()
}

fn regex_email(text: &str) -> bool {
    EMAIL_RE.is_match(text)
}

fn main() {
    let caps = regex_date("2019-03-24");

    assert_eq!("2019", &caps["year"]);
    assert_eq!("03", &caps["month"]);
    assert_eq!("24", &caps["day"]);

    let after = RE.replace_all("2019-03-24", "$month/$day/$year");
    assert_eq!(after, "03/24/2019");

    let after = RE.replace_all("2019-03-24", "$month/$notfoundday/$year");
    assert_eq!(after, "03//2019");

    assert_eq!(regex_email("63575@qq.com"), true);
    assert_eq!(regex_email("xxxx"), false);
}
