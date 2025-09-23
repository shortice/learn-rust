use std::result::Result;

fn ok_ne_ok(value: i64) -> Result<i64, String> {
    if value == 0 {
        return Err(String::from("Not zero."));
    }

    Ok(value)
}

fn printer(value: Result<i64, String>) {
    match value {
        Ok(value) => println!("Its ok! The value: {0}", value),
        Err(err) => println!("Not ok, the error: {0}", err),
    }
}

pub fn is_okay(value: i64) {
    printer(ok_ne_ok(value));
}
