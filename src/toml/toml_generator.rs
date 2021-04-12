use std::fs::File;
use std::io::prelude::*;
use toml::Value;

pub enum GenerateFailed {
  ConvertFailed(String),
  FileError(String),
}

pub fn generate_toml(
  file_name: &str,
  toml_contents: &mut Value
) -> Result<(), GenerateFailed> {

  let mut comb_build;
  match File::create(file_name) {
    Err(_) => return Err(GenerateFailed::FileError(format!("couldn't create file {}", file_name))),
    Ok(file) => comb_build = file,
  }

  let mut comb_build_contents = "# This file was created by comb as part of the build process.\n# It is not to be edited by the user.\n\n".to_string();
  let mut comb_build_generator = Value::Table(toml::value::Map::new());

  comb_build_generator.as_table_mut().unwrap().insert("module".to_string(), toml_contents.as_table().unwrap().get("module").unwrap().clone());
  comb_build_contents.push_str(&comb_build_generator.to_string());
  comb_build_contents.push('\n');

  comb_build_generator.as_table_mut().unwrap().remove("module");
  comb_build_generator.as_table_mut().unwrap().insert("dependencies".to_string(), toml_contents.as_table().unwrap().get("dependencies").unwrap().clone());
  comb_build_contents.push_str(&comb_build_generator.to_string());
  comb_build_contents.push('\n');

  match comb_build.write_all(comb_build_contents.as_bytes()) {
    Err(_) => return Err(GenerateFailed::FileError(format!("failed to write to {}", file_name))),
    Ok(_) => (),
  }

  Ok(())
}