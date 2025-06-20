use fancy_regex::Regex;
use nu_engine::command_prelude::*;

pub fn split_str(s: &str, regex: &Regex, max_split: Option<usize>, collapse_empty: bool, v_span: Span) -> Vec<Value>
{
    match max_split {
        Some(max_split) => regex
            .splitn(&s, max_split)
            .map(|x| match x {
                Ok(val) => Value::string(val, v_span),
                Err(err) => Value::error(
                    ShellError::GenericError {
                        error: "Error with regular expression".into(),
                        msg: err.to_string(),
                        span: Some(v_span),
                        help: None,
                        inner: vec![],
                    },
                    v_span,
                ),
            })
            .filter(|x| !(collapse_empty && x.is_empty()))
            .collect(),
        None => regex
            .split(&s)
            .map(|x| match x {
                Ok(val) => Value::string(val, v_span),
                Err(err) => Value::error(
                    ShellError::GenericError {
                        error: "Error with regular expression".into(),
                        msg: err.to_string(),
                        span: Some(v_span),
                        help: None,
                        inner: vec![],
                    },
                    v_span,
                ),
            })
            .filter(|x| !(collapse_empty && x.is_empty()))
            .collect(),
    }
}

// -        let split_result: Vec<_> = match max_split {
// -            Some(max_split) => separator
// -                .splitn(&s, max_split)
// -                .filter_map(|x| x.ok())
// -                .filter(|x| !(collapse_empty && x.is_empty()))
// -                .collect(),
// -            None => separator
// -                .split(&s)
// -                .filter_map(|x| x.ok())
// -                .filter(|x| !(collapse_empty && x.is_empty()))
// -                .collect(),
