pub mod input;
mod playerdata;
mod text;

#[cfg(test)]
mod tests {
    use crate::playerdata::ArgType;
    #[test]
    fn char_parse() {
        let arg: ArgType = "falcon".parse().unwrap();
        let parsed = ArgType::Character(0);
        assert_eq!(parsed, arg);
    }
    #[test]
    fn char_parse2() {
        let arg: ArgType = "captain falcon".parse().unwrap();
        let parsed = ArgType::Character(0);
        assert_eq!(parsed, arg);
    }
    #[test]
    fn char_parse3() {
        let parsed1: ArgType = "captain falcon".parse().unwrap();
        let parsed2: ArgType = "falcon".parse().unwrap();
        assert_eq!(parsed1, parsed2);
    }
    #[test]
    fn stage_parse() {
        let arg: ArgType = "fod".parse().unwrap();
        let parsed = ArgType::Stage(2);
        assert_eq!(arg, parsed);
    }
}
