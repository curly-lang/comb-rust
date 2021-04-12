use super::toml_parser;
use super::toml_completeness;
use super::toml_generator;

use std::path::Path;

pub fn generate_build_toml(
  input_file_name: &str,
  output_file_name: &str,
  update: bool,
) -> Result<(), ()> {
  if Path::new(output_file_name).exists() && !update {
    println!("File {} exists, skipping regeneration...", output_file_name);
    return Ok(());
  }

  let parsed_toml_attempt = toml_parser::parse_toml(input_file_name);
  let mut parsed_toml;

  match parsed_toml_attempt {
    Ok(toml) => {
      parsed_toml = toml;
    }
    Err(e) => {
      match e {
        toml_parser::ParseFailed::FileError(e) => {
          println!(
            "Error reading {}:\n{}",
            input_file_name,
            e.to_string()
          )
        }
        toml_parser::ParseFailed::TomlError(e) => {
          println!(
            "Error parsing {} at line {}, char {}:\n{}",
            input_file_name,
            e.line_col().unwrap().0, e.line_col().unwrap().1,
            e.to_string()
          )
        }
      }
      return Err(());
    }
  }

  let completed_toml_attempt = toml_completeness::complete_module (&mut parsed_toml);
  match completed_toml_attempt {
    Err(e) => {
      match e {
        toml_completeness::CompleteError::NotEnoughInfo(e) => {
          println!(
            "Missing information in {}:\n{}",
            input_file_name,
            e
          )
        }
        toml_completeness::CompleteError::BadTomlType(e) => {
          println!(
            "Invalid type for a build parameter in {}:\n{}",
            input_file_name,
            e
          )
        }
        toml_completeness::CompleteError::DependencyFail(e) => {
          println!(
            "Error with one of the dependencies in {}:\n{}",
            input_file_name,
            e
          )
        }
      }
      return Err(());
    }
    _ => ()
  }



  let generate_toml_attempt = toml_generator::generate_toml(output_file_name, &mut parsed_toml);
  match generate_toml_attempt {
    Err(e) => {
      match e {
        toml_generator::GenerateFailed::FileError(e) => println!("Failed to write to {}:\n{}", output_file_name, e),
        toml_generator::GenerateFailed::ConvertFailed(e) => println!("Error converting the comb-created build toml to a string:\n{}", e),
      }
      return Err(());
    }
    _ => ()
  }

  return Ok(());
}