use nom::bytes::complete::tag;

#[derive(Clone, Default, Debug)]
pub struct Mount {
    pub device: std::string::String,
    pub mount_point: std::string::String,
    pub file_system_type: std::string::String,
    pub options: std::vec::Vec<std::string::String>,
}

/*
A tag parser recognizes a literal string, or "tag", of text. The tag parser tag("hello") 
is a function object that recognizes the text "hello". We then call the tag parser with 
the input string as its argument and return the result.
 */
fn hello_parser(i: &str) -> nom::IResult<&str, &str> {
    tag("hello")(i)
}

fn not_whitespace(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::is_not(" \t")(i)
}

#[cfg(test)]
mod tests {
    use nom::error::Error;

    use super::*;
    #[test]
    fn test_hello_parser() {
        println!("{:?}", hello_parser("hello"));
        println!("{:?}", hello_parser("hello world"));
        println!("{:?}", hello_parser("goodbye hello again"));
    }
    #[test]
    fn test_not_whitespace() {
        assert_eq!(not_whitespace("abcd efg"), Ok((" efg", "abcd")));
        assert_eq!(not_whitespace("abcd\tefg"), Ok(("\tefg", "abcd")));
        println!("{:?}", not_whitespace(" abcdefg"));
    }
}
