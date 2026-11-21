//! Read-only parser for Steam KeyValues manifests. It never reads credentials or launches Steam.
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value { String(String), Object(BTreeMap<String, Value>) }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    UnterminatedString, InvalidEscape, UnexpectedEnd, UnexpectedToken, DuplicateKey(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token { String(String), Open, Close }

fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut tokens = Vec::new();
    while i < chars.len() {
        match chars[i] {
            c if c.is_whitespace() || c == '\u{feff}' => i += 1,
            '/' if i + 1 < chars.len() && chars[i + 1] == '/' => {
                i += 2;
                while i < chars.len() && chars[i] != '\n' { i += 1; }
            }
            '{' => { tokens.push(Token::Open); i += 1; }
            '}' => { tokens.push(Token::Close); i += 1; }
            '"' => {
                i += 1;
                let mut value = String::new();
                let mut closed = false;
                while i < chars.len() {
                    match chars[i] {
                        '"' => { i += 1; closed = true; break; }
                        '\\' => {
                            i += 1;
                            if i >= chars.len() { return Err(ParseError::UnterminatedString); }
                            match chars[i] {
                                '"' => value.push('"'),
                                '\\' => value.push('\\'),
                                'n' => value.push('\n'),
                                't' => value.push('\t'),
                                other => { value.push('\\'); value.push(other); }
                            }
                            i += 1;
                        }
                        c => { value.push(c); i += 1; }
                    }
                }
                if !closed { return Err(ParseError::UnterminatedString); }
                tokens.push(Token::String(value));
            }
            _ => return Err(ParseError::UnexpectedToken),
        }
    }
    Ok(tokens)
}

fn parse_object(tokens: &[Token], cursor: &mut usize, nested: bool)
    -> Result<BTreeMap<String, Value>, ParseError> {
    let mut values = BTreeMap::new();
    loop {
        match tokens.get(*cursor) {
            None if nested => return Err(ParseError::UnexpectedEnd),
            None => return Ok(values),
            Some(Token::Close) if nested => { *cursor += 1; return Ok(values); }
            Some(Token::Close) => return Err(ParseError::UnexpectedToken),
            Some(Token::String(_)) => {}
            _ => return Err(ParseError::UnexpectedToken),
        }
        let key = match tokens.get(*cursor) {
            Some(Token::String(key)) => key.clone(),
            _ => return Err(ParseError::UnexpectedToken),
        };
        *cursor += 1;
        let value = match tokens.get(*cursor) {
            Some(Token::String(value)) => {
                *cursor += 1;
                Value::String(value.clone())
            }
            Some(Token::Open) => {
                *cursor += 1;
                Value::Object(parse_object(tokens, cursor, true)?)
            }
            _ => return Err(ParseError::UnexpectedToken),
        };
        if values.insert(key.clone(), value).is_some() {
            return Err(ParseError::DuplicateKey(key));
        }
    }
}

pub fn parse_vdf(input: &str) -> Result<BTreeMap<String, Value>, ParseError> {
    let tokens = tokenize(input)?;
    let mut cursor = 0;
    let result = parse_object(&tokens, &mut cursor, false)?;
    if cursor != tokens.len() { return Err(ParseError::UnexpectedToken); }
    Ok(result)
}

fn string<'a>(map: &'a BTreeMap<String, Value>, key: &str) -> Option<&'a str> {
    match map.get(key)? { Value::String(value) => Some(value), _ => None }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteamAppManifest {
    pub app_id: String,
    pub name: String,
    pub install_directory: String,
    pub state_flags: Option<String>,
}

impl SteamAppManifest {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let root = parse_vdf(input)?;
        let app = match root.get("AppState") {
            Some(Value::Object(map)) => map,
            _ => return Err(ParseError::UnexpectedToken),
        };
        Ok(Self {
            app_id: string(app, "appid").ok_or(ParseError::UnexpectedToken)?.to_owned(),
            name: string(app, "name").ok_or(ParseError::UnexpectedToken)?.to_owned(),
            install_directory: string(app, "installdir").ok_or(ParseError::UnexpectedToken)?.to_owned(),
            state_flags: string(app, "StateFlags").map(str::to_owned),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteamLibraryFolder {
    pub path: String,
    pub app_ids: Vec<String>,
}

pub fn parse_library_folders(input: &str) -> Result<Vec<SteamLibraryFolder>, ParseError> {
    let root = parse_vdf(input)?;
    let libraries = match root.get("libraryfolders") {
        Some(Value::Object(map)) => map,
        _ => return Err(ParseError::UnexpectedToken),
    };
    let mut result = Vec::new();
    for value in libraries.values() {
        let Value::Object(fields) = value else { continue };
        let Some(path) = string(fields, "path") else { continue };
        let app_ids = match fields.get("apps") {
            Some(Value::Object(apps)) => apps.keys().cloned().collect(),
            _ => Vec::new(),
        };
        result.push(SteamLibraryFolder { path: path.to_owned(), app_ids });
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_nested_objects_and_comments() {
        let parsed = parse_vdf("// header\n\"root\" { \"child\" { \"key\" \"value\" } }").unwrap();
        assert!(matches!(parsed.get("root"), Some(Value::Object(_))));
    }
    #[test]
    fn parses_escaped_quotes_and_backslashes() {
        let parsed = parse_vdf("\"root\" { \"text\" \"say \\\"hi\\\" \\\\ folder\" }").unwrap();
        let root = match parsed.get("root").unwrap() { Value::Object(map) => map, _ => panic!() };
        assert_eq!(string(root, "text"), Some("say \"hi\" \\ folder"));
    }
    #[test]
    fn rejects_duplicate_keys() {
        assert_eq!(parse_vdf("\"a\" \"1\" \"a\" \"2\""),
            Err(ParseError::DuplicateKey("a".into())));
    }
    #[test]
    fn parses_app_manifest() {
        let text = "\"AppState\" { \"appid\" \"570\" \"name\" \"Dota 2\" \"installdir\" \"dota 2\" \"StateFlags\" \"4\" }";
        let app = SteamAppManifest::parse(text).unwrap();
        assert_eq!(app.app_id, "570");
        assert_eq!(app.name, "Dota 2");
        assert_eq!(app.install_directory, "dota 2");
    }
    #[test]
    fn parses_library_folders_and_app_ids() {
        let text = "\"libraryfolders\" { \"0\" { \"path\" \"/Steam\" \"apps\" { \"10\" \"1\" \"20\" \"1\" } } }";
        let folders = parse_library_folders(text).unwrap();
        assert_eq!(folders[0].path, "/Steam");
        assert_eq!(folders[0].app_ids, vec!["10", "20"]);
    }
}
