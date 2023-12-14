// https://bodil.lol/parser-combinators/

/*
<parent-element>
  <single-element attribute="value" />
</parent-element>
*/

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}

fn the_letter_a(input: &str) -> Result<(&str, ()), &str> {
    match input.chars().next() {
        Some('a') => Ok((&input['a'.len_utf8()..], ())),
        _ => Err(input),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_letter_a() {
        let res = the_letter_a("abcd");
        println!("{:#?}", res);
    }
}
