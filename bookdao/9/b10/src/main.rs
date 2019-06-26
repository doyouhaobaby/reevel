fn main() {
    let number: f64 = 20.;
    let result = Option::from(number)
        .map(inverse);
    assert_eq!(result, Option::from(-20.));

    let result = result.map(double);
    assert_eq!(result, Option::from(-40.));

    let result = result.map(inverse);
    assert_eq!(result, Option::from(40.));

    let result = result.and_then(log);
    assert_eq!(result, Option::from(5.321928094887363));

    let result = result.map(square);
    assert_eq!(result, Option::from(28.322918647151432));

    let result = result.and_then(sqrt);
    assert_eq!(result, Option::from(5.321928094887363));
}

fn inverse(value: f64) ->f64 {
    value * -1.
}

fn square(value: f64) ->f64 {
    value.powi(2 as i32)
}

fn double(value: f64) ->f64 {
    value * 2.
}

fn log(value: f64) -> Option<f64> {
    match value.log2() {
        x if x.is_normal() => Some(x),
        _ => None,
    }
}

fn sqrt(value: f64) -> Option<f64> {
    match value.sqrt() {
        x if x.is_normal() => Some(x),
        _ => None,
    }
}
