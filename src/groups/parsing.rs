use super::*;

// Trim whitespaces at the beginning of the string
fn skip_whitespaces(s: &str) -> &str {
    s.trim_start()
}

// Trim an expected prefix at the beginning of the string
fn expect<'a, 'b>(s: &'a str, expected: &'b str) -> Result<&'a str, (&'a str, String)> {
    if s.starts_with(expected) {
        Ok(&s[expected.len()..])
    } else {
        Err((s, format!("Expected {expected:?}.")))
    }
}

// Read an identifier at the beginning of the string
fn read_identifier(s: &str) -> Result<(&str, String), (&str, String)> {
    let mut i = 0;
    for c in s.chars() {
        if !c.is_ascii_alphanumeric() && c != ':' && c != '_' && c != '-' {
            break;
        }
        i += 1;
    }
    if i == 0 {
        return Err((s, String::from("Expected non-empty identifier.")));
    }
    Ok((&s[i..], s[..i].to_string()))
}

// Read a simple filter `name=value` at the beginning of the string
fn read_simple_filter(s: &str) -> Result<(&str, GroupFilter), (&str, String)> {
    let s = skip_whitespaces(s);
    let (s, id) = read_identifier(s)?;
    let s = skip_whitespaces(s);
    let s = expect(s, "=")?;
    let s = skip_whitespaces(s);
    let (s, value) = read_identifier(s)?;
    Ok((s, GroupFilter::Is { id, value }))
}

// Read a composite filter `(name=value AND name=value)` at the beginning of the string
fn read_composite_filter(s: &str) -> Result<(&str, GroupFilter), (&str, String)> {
    let s = skip_whitespaces(s);
    let mut s = expect(s, "(")?;
    let mut is_or = false;
    let mut filters = Vec::new();
    loop {
        if s.starts_with(')') {
            break;
        }
        let (ns, filter) = read_filter(s)?;
        filters.push(filter);
        let ns = skip_whitespaces(ns);
        s = ns;
        if s.starts_with(')') {
            break;
        }
        if s.starts_with("OR ") || s.starts_with("or ") {
            if filters.len() == 1 {
                is_or = true;
            } else if !is_or {
                return Err((s, String::from("OR filter mixed with AND filter.")));
            }
            s = &s[3..];
            continue;
        }
        if s.starts_with("AND ") || s.starts_with("and ") {
            if filters.len() == 1 {
                is_or = false;
            } else if is_or {
                return Err((s, String::from("AND filter mixed with OR filter.")));
            }
            s = &s[4..];
            continue;
        }
        return Err((s, format!("Expected OR, AND or a closing parenthesis at the end of a composite filter. Got {s:?}")));
    }
    let s = expect(s, ")")?;
    
    if filters.len() == 1 {
        return Ok((s, filters.pop().unwrap()));
    }
    match is_or {
        true => Ok((s, GroupFilter::Any(filters))),
        false => Ok((s, GroupFilter::All(filters))),
    }
}

// Read a filter, either simple or composite, at the beginning of the string
fn read_filter(s: &str) -> Result<(&str, GroupFilter), (&str, String)> {
    let (s1, e1) = match read_simple_filter(s) {
        Ok((s, filter)) => return Ok((s, filter)),
        Err(e) => e,
    };
    let p1 = s.len() - s1.len();
    let (s2, e2) = match read_composite_filter(s) {
        Ok((s, filter)) => return Ok((s, filter)),
        Err(e) => e,
    };
    let p2 = s.len() - s2.len();
    Err((s, format!("Expected a simple or a composite filter. Both result in reading errors. Simple filter read, at char {p1}: {e1:?}, Composite filter read, at char {p2}: {e2:?}")))
}

// Read the whole string as a filter
pub(crate) fn read_whole_as_filter(s: &str) -> Result<GroupFilter, (&str, String)> {
    let (s, filter) = read_filter(s)?;
    let s = skip_whitespaces(s);
    if s.is_empty() {
        Ok(filter)
    } else {
        Err((s, format!("Expected end of string. Got something after the filter {s:?}")))
    }
}

#[cfg(test)]
#[test]
fn test() {
    read_whole_as_filter("test=value").unwrap();
    read_whole_as_filter("()").unwrap();
    read_whole_as_filter("(test=value AND other=value)").unwrap();
    read_whole_as_filter("(test=value OR other=value)").unwrap();
    read_whole_as_filter("(test=value OR (test=value AND other=value))").unwrap();
    read_whole_as_filter("(test=value   OR ( test=value   AND  other=value))").unwrap();

    read_whole_as_filter("(tést=value OR (test=value AND other=value))").unwrap_err();
    read_whole_as_filter("(test=value OR test=value AND other=value)").unwrap_err();
}
