#[derive(Debug)]
pub enum ParseFailed {
  FileError(Box<dyn std::error::Error>),
  TomlError(toml::de::Error),
}

pub fn parse_toml(
  file_name: &str
) -> Result<toml::Value, ParseFailed> {

  let file_read_attempt = std::fs::read_to_string(file_name);
  let file_contents;

  match file_read_attempt {
    Ok(contents) => file_contents = contents,
    Err(e) => return Err(ParseFailed::FileError(Box::new(e))),
  }

  let parsed_toml_attempt = file_contents.as_str().parse::<toml::Value>();
  match parsed_toml_attempt {
    Ok(parsed_toml) => Ok(parsed_toml),
    Err(e) => Err(ParseFailed::TomlError(e)),
  }
}