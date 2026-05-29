fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        assert_eq!("Hello, World!", hello_world::hello());
    }
}
