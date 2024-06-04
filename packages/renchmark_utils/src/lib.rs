pub fn parse_config(conf: String) -> String {
    return conf;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let str = "Test".to_string();
        assert_eq!(str.clone(), parse_config(str));
    }
}
