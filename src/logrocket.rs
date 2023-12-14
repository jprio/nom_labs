/*
Parser combinators are higher-order functions that can accept several parsers as input and return a new parser as its output.
This approach enables you to build parsers for simple tasks, such as parsing a certain string or a number, and compose
 them using combinator functions into a whole recursive descent parser.
Benefits of combinatory parsing include testability, maintainability, and readability; each moving part
 is rather small and self-isolated, making the whole parser a composition of modular components.
 */

use nom::{ error::{ context, VerboseError }, bytes::complete::tag_no_case, branch::alt, IResult };

#[derive(Debug, PartialEq, Eq)]
pub struct URI<'a> {
    scheme: Scheme,
    authority: Option<Authority<'a>>,
    host: Host,
    port: Option<u16>,
    path: Option<Vec<&'a str>>,
    query: Option<QueryParams<'a>>,
    fragment: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Scheme {
    HTTP,
    HTTPS,
}

pub type Authority<'a> = (&'a str, Option<&'a str>);

#[derive(Debug, PartialEq, Eq)]
pub enum Host {
    HOST(String),
    IP([u8; 4]),
}

pub type QueryParam<'a> = (&'a str, &'a str);
pub type QueryParams<'a> = Vec<QueryParam<'a>>;

impl From<&str> for Scheme {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "http://" => Scheme::HTTP,
            "https://" => Scheme::HTTPS,
            _ => unimplemented!("no other schemes supported"),
        }
    }
}
type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn scheme(input: &str) -> Res<&str, Scheme> {
    context(
        "scheme",
        alt((tag_no_case("HTTP://"), tag_no_case("HTTPS://")))
    )(input).map(|(next_input, res)| (next_input, res.into()))
}

#[cfg(test)]
mod tests {
    use nom::{ error::{ ErrorKind, VerboseError, VerboseErrorKind }, Err as NomErr };
    use super::*;
    #[test]
    fn test_scheme() {
        assert_eq!(scheme("https://yay"), Ok(("yay", Scheme::HTTPS)));
        assert_eq!(scheme("http://yay"), Ok(("yay", Scheme::HTTP)));
        assert_eq!(
            scheme("bla://yay"),
            Err(
                NomErr::Error(VerboseError {
                    errors: vec![
                        ("bla://yay", VerboseErrorKind::Nom(ErrorKind::Tag)),
                        ("bla://yay", VerboseErrorKind::Nom(ErrorKind::Alt)),
                        ("bla://yay", VerboseErrorKind::Context("scheme"))
                    ],
                })
            )
        );
    }
}
