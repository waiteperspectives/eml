use super::eventmodel::*;
use super::utils::newid;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::{alpha1, u32 as parse_u32};
use nom::character::complete::{line_ending, space0, space1};
use nom::multi::{many0, many1, separated_list0};
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

// parse version info
fn eml_prefix(input: &str) -> IResult<&str, EmlPrefix> {
    let hash = tag("#");
    let eml = tag("eml");
    let colon = tag(":");
    let (rest, _) = tuple((hash, space0, eml, colon, space0))(input)?;
    Ok((rest, EmlPrefix {}))
}

fn eml_version(input: &str) -> IResult<&str, Version> {
    let version_info = tuple((parse_u32, tag("."), parse_u32, tag("."), parse_u32));
    let (rest, (major, _, minor, _, fix)) = preceded(eml_prefix, version_info)(input)?;
    Ok((rest, Version { major, minor, fix }))
}

// parse expressions
fn textfield_key(input: &str) -> IResult<&str, &str> {
    let key_parser = alpha1;
    let (newinput, key) = terminated(key_parser, tag(":"))(input)?;
    Ok((newinput, key))
}

fn textfield_value(input: &str) -> IResult<&str, &str> {
    let parse_until_end = is_not(",\n}");
    let (rest, text) = preceded(space0, parse_until_end)(input)?;
    Ok((rest, text))
}

fn textfield(input: &str) -> IResult<&str, Field> {
    let (rest, _) = space0(input)?;
    let (rest, id) = textfield_key(rest)?;
    let (rest, text) = textfield_value(rest)?;
    Ok((
        rest,
        Field::Text(TextField {
            name: id.to_string(),
            data: text.to_string(),
        }),
    ))
}

fn fields(input: &str) -> IResult<&str, Vec<Field>> {
    let delimiter = alt((tag(","), line_ending));
    let field_parser = textfield;
    separated_list0(delimiter, field_parser)(input)
}

fn fields_block(input: &str) -> IResult<&str, Body> {
    let block_begin = terminated(tag("{"), alt((line_ending, space0)));
    let block_end = preceded(alt((line_ending, space0)), tag("}"));
    let (rest, fields) = delimited(block_begin, fields, block_end)(input)?;
    Ok((rest, Body::FieldBody(fields)))
}

fn raw_block(input: &str) -> IResult<&str, Body> {
    let block_begin = terminated(tag("{"), space0);
    let block_end = preceded(space0, tag("}"));
    let (rest, raw) = delimited(block_begin, take_until("}"), block_end)(input)?;
    let rawlines = raw
        .split("\n")
        .map(|x| x.trim_start().to_string())
        .collect::<Vec<String>>();
    Ok((rest, Body::TableBody(rawlines)))
}

fn use_block(input: &str) -> IResult<&str, Body> {
    let block_begin = terminated(tag("{"), space0);
    let block_end = preceded(space0, tag("}"));
    let useid = preceded(tag("use "), alpha1);
    let (rest, exprid) = delimited(block_begin, useid, block_end)(input)?;
    Ok((rest, Body::UseBody(ExpressionId(exprid.to_string()))))
}

fn expression_type(input: &str) -> IResult<&str, ExpressionType> {
    match alt((
        tag("form"),
        tag("job"),
        tag("command"),
        tag("event"),
        tag("view"),
        tag("flow"),
    ))(input)
    {
        Ok((rest, "form")) => Ok((rest, ExpressionType::Form)),
        Ok((rest, "job")) => Ok((rest, ExpressionType::Job)),
        Ok((rest, "command")) => Ok((rest, ExpressionType::Command)),
        Ok((rest, "event")) => Ok((rest, ExpressionType::Event)),
        Ok((rest, "view")) => Ok((rest, ExpressionType::View)),
        Ok((rest, "flow")) => Ok((rest, ExpressionType::Flow)),
        Ok((_, _)) => panic!("unreachable destination"), // TODO: return error
        Err(e) => Err(e),
    }
}

fn expression_id(input: &str) -> IResult<&str, ExpressionId> {
    let (rest, (_, id, _)) = tuple((space1, alpha1, space1))(input)?;
    Ok((rest, ExpressionId(id.to_string())))
}

fn flow_block(input: &str) -> IResult<&str, Vec<ExpressionId>> {
    let block_begin = terminated(tag("{"), space0);
    let block_end = preceded(space0, tag("}"));
    let expression = preceded(space0, alpha1);
    let arrow = preceded(space0, tag("=>"));
    let (rest, ids) = delimited(block_begin, separated_list0(arrow, expression), block_end)(input)?;
    let expressions = ids
        .iter()
        .map(|id| ExpressionId(id.to_string()))
        .collect::<Vec<ExpressionId>>();
    Ok((rest, expressions))
}

fn identified_block(input: &str) -> IResult<&str, (ExpressionId, Body)> {
    let (rest, exprid) = expression_id(input)?;
    let (rest, body) = alt((use_block, fields_block, raw_block))(rest)?;
    Ok((rest, (exprid, body)))
}

fn expression(input: &str) -> IResult<&str, Expression> {
    let (rest, exprtyp) = expression_type(input)?;
    match exprtyp {
        ExpressionType::Form => {
            let (rest, (exprid, body)) = identified_block(rest)?;
            Ok((rest, Expression::Form(exprid, body)))
        }
        ExpressionType::Job => {
            let (rest, (exprid, body)) = identified_block(rest)?;
            Ok((rest, Expression::Job(exprid, body)))
        }
        ExpressionType::Command => {
            let (rest, (exprid, body)) = identified_block(rest)?;
            Ok((rest, Expression::Command(exprid, body)))
        }
        ExpressionType::Event => {
            let (rest, (exprid, body)) = identified_block(rest)?;
            Ok((rest, Expression::Event(exprid, body)))
        }
        ExpressionType::View => {
            let (rest, (exprid, body)) = identified_block(rest)?;
            Ok((rest, Expression::View(exprid, body)))
        }
        ExpressionType::Flow => {
            let (rest, ids) = preceded(space0, flow_block)(rest)?;
            let exprid = ExpressionId(newid());
            Ok((rest, Expression::Flow(exprid, ids)))
        }
    }
}

fn expressions(input: &str) -> IResult<&str, Vec<Expression>> {
    let expression_parser = expression;
    let delimiter = many1(line_ending);
    let (rest, expressions) = separated_list0(delimiter, expression_parser)(input)?;
    Ok((rest, expressions))
}

pub fn parse(input: &str) -> Result<EventModel, String> {
    let (rest, _version) = match eml_version(input) {
        Ok((rest, version)) => (rest, version),
        _ => return Err("bad version".to_string()),
    };

    let mut parser = preceded(many0(line_ending), expressions);

    match parser(rest) {
        Ok((_, expressions)) => Ok(EventModel { expressions }),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_eml_version() {
        let input = "# eml: 0.0.1";
        let expected = Version {
            major: 0,
            minor: 0,
            fix: 1,
        };
        let (_, observed) = eml_version(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_textfield() {
        let input = "foo: bar";
        let expected = Field::Text(TextField {
            name: "foo".to_string(),
            data: "bar".to_string(),
        });
        let (_, observed) = textfield(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_fields_newline() {
        let input = "foo: bar\n    baz: ooka";
        let expected = vec![
            Field::Text(TextField {
                name: "foo".to_string(),
                data: "bar".to_string(),
            }),
            Field::Text(TextField {
                name: "baz".to_string(),
                data: "ooka".to_string(),
            }),
        ];
        let (_, observed) = fields(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_fields_csv() {
        let input = "foo: bar, baz: ooka";
        let expected = vec![
            Field::Text(TextField {
                name: "foo".to_string(),
                data: "bar".to_string(),
            }),
            Field::Text(TextField {
                name: "baz".to_string(),
                data: "ooka".to_string(),
            }),
        ];
        let (_, observed) = fields(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_fields_block_multiline() {
        let input = "{\n  foo: bar, baz: ooka\n}";
        let expected = Body::FieldBody(vec![
            Field::Text(TextField {
                name: "foo".to_string(),
                data: "bar".to_string(),
            }),
            Field::Text(TextField {
                name: "baz".to_string(),
                data: "ooka".to_string(),
            }),
        ]);
        let (_, observed) = fields_block(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_fields_block_inline() {
        let input = "{foo: bar, baz: ooka}";
        let expected = Body::FieldBody(vec![
            Field::Text(TextField {
                name: "foo".to_string(),
                data: "bar".to_string(),
            }),
            Field::Text(TextField {
                name: "baz".to_string(),
                data: "ooka".to_string(),
            }),
        ]);
        let (_, observed) = fields_block(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_flow_block() {
        let input = "{Foo =>Bar => Baz }";
        let expected = vec![
            ExpressionId("Foo".to_string()),
            ExpressionId("Bar".to_string()),
            ExpressionId("Baz".to_string()),
        ];
        let (_, observed) = flow_block(input).unwrap();
        assert_eq!(expected, observed);
    }

    #[test]
    fn test_use_block_01() {
        let input = "{ use FooBar}";
        let expected = Body::UseBody(ExpressionId("FooBar".to_string()));
        let (_, observed) = use_block(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_use_block_02() {
        let input = "{use FooBar}";
        let expected = Body::UseBody(ExpressionId("FooBar".to_string()));
        let (_, observed) = use_block(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_use_block_03() {
        let input = "{use FooBar }";
        let expected = Body::UseBody(ExpressionId("FooBar".to_string()));
        let (_, observed) = use_block(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_parse_body_01() {
        let input = "form FooForm {}";
        let expected = vec![Expression::Form(
            ExpressionId("FooForm".to_string()),
            Body::FieldBody(vec![]),
        )];
        let (_, observed) = expressions(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_parse_body_02() {
        let input = "form FooForm { foo:bar}";
        let expected = vec![Expression::Form(
            ExpressionId("FooForm".to_string()),
            Body::FieldBody(vec![Field::Text(TextField {
                name: "foo".to_string(),
                data: "bar".to_string(),
            })]),
        )];
        let (_, observed) = expressions(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_parse_body_03() {
        let input = indoc! {r#"
            form FooForm {
                foo: bar
            }
            command AddBar {
                foo: bar
            }

        "#};
        let expected = vec![
            Expression::Form(
                ExpressionId("FooForm".to_string()),
                Body::FieldBody(vec![Field::Text(TextField {
                    name: "foo".to_string(),
                    data: "bar".to_string(),
                })]),
            ),
            Expression::Command(
                ExpressionId("AddBar".to_string()),
                Body::FieldBody(vec![Field::Text(TextField {
                    name: "foo".to_string(),
                    data: "bar".to_string(),
                })]),
            ),
        ];
        let (_, observed) = expressions(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_parse_body_04() {
        let input = indoc! {r#"
            form FooForm {
                foo: bar
            }
            command AddBar { use FooForm}
        "#};
        let expected = vec![
            Expression::Form(
                ExpressionId("FooForm".to_string()),
                Body::FieldBody(vec![Field::Text(TextField {
                    name: "foo".to_string(),
                    data: "bar".to_string(),
                })]),
            ),
            Expression::Command(
                ExpressionId("AddBar".to_string()),
                Body::UseBody(ExpressionId("FooForm".to_string())),
            ),
        ];
        let (_, observed) = expressions(input).unwrap();
        assert_eq!(expected, observed)
    }

    #[test]
    fn test_parse_body_05() {
        let input = indoc! {r#"
            view TodoList {
              | CustomerId | state  |
              |------------|--------|
              | 123        | done   |
              | 456        | todo   |
              | 789        | todo   |
            }
        "#};
        let expected = vec![Expression::View(
            ExpressionId("TodoList".to_string()),
            Body::TableBody(vec![
                "".to_string(),
                "| CustomerId | state  |".to_string(),
                "|------------|--------|".to_string(),
                "| 123        | done   |".to_string(),
                "| 456        | todo   |".to_string(),
                "| 789        | todo   |".to_string(),
                "".to_string(),
            ]),
        )];
        let (_, observed) = expressions(input).unwrap();
        assert_eq!(expected, observed)
    }
}
