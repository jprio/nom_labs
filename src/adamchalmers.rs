use nom::{
    character::complete::{ digit1, alpha1 },
    character::complete::{ char, alpha0 },
    IResult,
    Err,
    bytes::complete::tag,
};

fn digit1_parser(input: &str) -> IResult<&str, &str> {
    digit1(input)
}
fn alpha1_parser(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}
fn alpha0_parser(input: &str) -> IResult<&str, &str> {
    alpha0(input)
}

fn hello_parser(s: &str) -> IResult<&str, &str> {
    tag("Hello")(s)
}

#[cfg(test)]
mod tests {
    use nom::{
        error::{ Error, ErrorKind },
        sequence::{ pair, separated_pair },
        combinator::map,
        multi::{ many0, many1, many_till },
        branch::alt,
        character::{ complete::{ alpha1, alphanumeric1, alphanumeric0, anychar } },
        bytes::complete::take_until,
    };

    use super::*;
    #[test]
    fn test_digit1_parser() {
        assert_eq!(digit1_parser("21c"), Ok(("c", "21")));
        assert_eq!(digit1_parser("c1"), Err(Err::Error(Error::new("c1", ErrorKind::Digit))));
        assert_eq!(digit1_parser(""), Err(Err::Error(Error::new("", ErrorKind::Digit))));
    }
    #[test]
    fn test_hello_parser() {
        println!("{:?}", hello_parser("Hello lo"));
        assert_eq!(hello_parser("Hello lo"), Ok((" lo", "Hello")));
    }
    #[test]
    fn test_pair() {
        let mut pair_parser = pair(hello_parser, digit1_parser);
        println!("{:?}", pair_parser("Hello1"));
        // assert_eq!()
    }
    #[test]
    fn test_separated_pair() {
        let mut separated_pair_parser = separated_pair(hello_parser, char(' '), digit1_parser);
        println!("{:?}", separated_pair_parser("Hello 1"));
        assert_eq!(separated_pair_parser("Hello 1"), Ok(("", ("Hello", "1"))));
    }
    #[test]
    fn test_map() {
        let pair_parser = pair(hello_parser, digit1_parser);
        let mut map_parser = map(pair_parser, |(first, last)| first.to_owned() + "plop" + last);
        println!("{:?}", map_parser("Hello1"));
        assert_eq!(map_parser("Hello1").unwrap(), ("", "Helloplop1".to_owned()));
    }

    #[test]
    fn test_many() {
        // let mut parser = many0(alt((hello_parser, digit1_parser)));
        // let mut parser = many1(alt((hello_parser, alpha1, digit1_parser)));
        let mut parser = many0(alt((alpha1_parser, digit1_parser)));
        println!("{:?}", parser("2Hello11HellojHello23").unwrap().1);
    }
    #[test]
    fn test_hellojhello() {
        // let mut parser = many0(alt((hello_parser, digit1_parser)));
        // let mut parser = many1(alt((hello_parser, alpha1, digit1_parser)));
        let mut parser = many0(alt((tag("hello"), alpha1_parser)));
        println!("{:?}", parser("khellojhello").unwrap());
    }
}
