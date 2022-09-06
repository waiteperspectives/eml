use super::eventmodel::*;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::u32 as parse_u32;
use nom::character::complete::{line_ending, space0};
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// parse version info in the first line
#[derive(Debug, PartialEq)]
struct Version {
    major: u32,
    minor: u32,
    fix: u32,
}

struct EmlPrefix();

fn parse_prefix(input: &str) -> IResult<&str, EmlPrefix> {
    let hash = tag("#");
    let eml = tag("eml");
    let colon = tag(":");
    let (newinput, _) = tuple((hash, space0, eml, colon, space0))(input)?;
    Ok((newinput, EmlPrefix {}))
}

fn parse_version_info(input: &str) -> IResult<&str, Version> {
    let (newinput, (major, _, minor, _, fix)) =
        tuple((parse_u32, tag("."), parse_u32, tag("."), parse_u32))(input)?;
    Ok((newinput, Version { major, minor, fix }))
}

fn parse_version(input: &str) -> IResult<&str, Version> {
    preceded(parse_prefix, parse_version_info)(input)
}

// parse expressions
fn parse_textfield_text(input: &str) -> IResult<&str, &str> {
    let parse_until_quote = is_not("\"");
    let (newinput, text) =
        preceded(space0, delimited(tag("\""), parse_until_quote, tag("\"")))(input)?;
    Ok((newinput, text))
}

fn parse_textfield_id(input: &str) -> IResult<&str, &str> {
    let parse_until_colon = is_not(":");
    let (newinput, key) = terminated(parse_until_colon, tag(":"))(input)?;
    Ok((newinput, key))
}

fn parse_textfield(input: &str) -> IResult<&str, TextField> {
    let (dedented_input, _) = space0(input)?;
    let (input_without_textfield_id, id) = parse_textfield_id(dedented_input)?;
    let (newinput, text) = parse_textfield_text(input_without_textfield_id)?;
    // Ok((newinput, TextField { id, text }))
    // TODO: use &str
    Ok((
        newinput,
        TextField {
            name: id.to_string(),
            data: text.to_string(),
        },
    ))
}

fn parse_fields(input: &str) -> IResult<&str, Vec<TextField>> {
    let delimiter = alt((tag(","), line_ending));
    separated_list0(delimiter, parse_textfield)(input)
}

fn parse_block(input: &str) -> IResult<&str, Vec<TextField>> {
    let block_begin = terminated(tag("{"), alt((line_ending, space0)));
    let block_end = preceded(alt((line_ending, space0)), tag("}"));
    delimited(block_begin, parse_fields, block_end)(input)
}

pub fn parse(input: &str) -> Result<EventModel, String> {
    match parse_version(input) {
        Ok((body, version)) => (body, version),
        _ => return Err("bad version".to_string()),
    };

    Ok(EventModel {
        expressions: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        let input = "# eml: 0.0.1";
        let expected = Version {
            major: 0,
            minor: 0,
            fix: 1,
        };
        let (_, observed) = parse_version(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_parse_textfield() {
        let input = "foo: \"bar\"";
        let expected = TextField {
            name: "foo".to_string(),
            data: "bar".to_string(),
        };
        let (_, observed) = parse_textfield(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_parse_fields_newline() {
        let input = "foo: \"bar\"\n    baz: \"ooka\"";
        let expected = vec![
            TextField {
                name: "foo".to_string(),
                data: "bar".to_string(),
            },
            TextField {
                name: "baz".to_string(),
                data: "ooka".to_string(),
            },
        ];
        let (_, observed) = parse_fields(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_parse_fields_csv() {
        let input = "foo: \"bar\", baz: \"ooka\"";
        let expected = vec![
            TextField {
                name: "foo".to_string(),
                data: "bar".to_string(),
            },
            TextField {
                name: "baz".to_string(),
                data: "ooka".to_string(),
            },
        ];
        let (_, observed) = parse_fields(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_parse_block_multiline() {
        let input = "{\n  foo: \"bar\", baz: \"ooka\"\n}";
        let expected = vec![
            TextField {
                name: "foo".to_string(),
                data: "bar".to_string(),
            },
            TextField {
                name: "baz".to_string(),
                data: "ooka".to_string(),
            },
        ];
        let (_, observed) = parse_block(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_parse_block_inline() {
        let input = "{foo: \"bar\", baz: \"ooka\"}";
        let expected = vec![
            TextField {
                name: "foo".to_string(),
                data: "bar".to_string(),
            },
            TextField {
                name: "baz".to_string(),
                data: "ooka".to_string(),
            },
        ];
        let (_, observed) = parse_block(input).unwrap();
        assert_eq!(expected, observed)
    }
}
