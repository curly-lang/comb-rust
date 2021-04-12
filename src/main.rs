use comb::toml::toml_parser;
use comb::toml::toml_completeness;
use comb::toml::toml_generator;

fn main() {
  let parsed_toml_attempt = toml_parser::parse_toml("comb.toml");
  let mut parsed_toml;

  match parsed_toml_attempt {
    Ok(toml) => {
      parsed_toml = toml;
    }
    Err(e) => {
      match e {
        toml_parser::ParseFailed::FileError(e) => {
          println!(
            "Error reading comb.toml:\n{}",
            e.to_string()
          )
        }
        toml_parser::ParseFailed::TomlError(e) => {
          println!(
            "Error parsing comb.toml at line {}, char {}:\n{}",
            e.line_col().unwrap().0, e.line_col().unwrap().1,
            e.to_string()
          )
        }
      }
      return;
    }
  }

  let completed_toml_attempt = toml_completeness::complete(&mut parsed_toml);
  match completed_toml_attempt {
    Err(e) => {
      match e {
        toml_completeness::CompleteError::NotEnoughInfo(e) => println!("Missing information in comb.toml:\n{}", e),
        toml_completeness::CompleteError::BadTomlType(e) => println!("Invalid type for a build parameter in comb.toml:\n{}", e),
        toml_completeness::CompleteError::DependencyFail(e) => println!("Error with one of the dependencies in comb.toml:\n{}", e),
      }
      return;
    }
    _ => ()
  }

  let generate_toml_attempt = toml_generator::generate_toml(&mut parsed_toml);
  match generate_toml_attempt {
    Err(e) => {
      match e {
        toml_generator::GenerateFailed::FileError(e) => println!("Failed to write to comb.toml.gen:\n{}", e),
        toml_generator::GenerateFailed::ConvertFailed(e) => println!("Error with one of the dependencies in comb.toml\n{}", e),
      }
      return;
    }
    _ => ()
  }
}
