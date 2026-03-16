fn greet(name: &'static str) -> String {
    return format!("Hello, {}!", name);
}

fn main() {
    println!("{}", greet("name"));
}

#[cfg(test)]
mod tests {
    use crate::greet;

    #[test]
    fn greet_with_name() {
        let name = "name";
        assert_eq!(greet(name), "Hello, name!");
    }
}
