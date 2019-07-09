use std::str::FromStr;

pub fn get<T: FromStr>(key: &str, default: T) -> T {
    match std::env::var(key) {
        Ok(v) => v.parse::<T>().unwrap_or(default),
        Err(_) => default,
    }
}

pub fn must_get<T: FromStr>(key: &str) -> T {
    match dotenv::var(key) {
        Ok(v) => v
            .parse::<T>()
            .unwrap_or_else(|_| panic!(format!("{} parse failed", key))),
        _ => panic!(format!("{} is required", key)),
    }
}
