use std::fs::File;
use std::io::prelude::*;
use toml::Value;

pub enum GenerateFailed {
  ConvertFailed(String),
  FileError(String),
}

pub fn generate_toml(
  toml_contents: &mut Value
) -> Result<(), GenerateFailed> {

  let mut comb_build;
  match File::create("comb.build.toml") {
    Err(_) => return Err(GenerateFailed::FileError("couldn't create file comb.build".to_string())),
    Ok(file) => comb_build = file,
  }

  let mut comb_build_contents = "".to_string();
  let mut comb_build_generator = Value::Table(toml::value::Map::new());

  comb_build_generator.as_table_mut().unwrap().insert("module".to_string(), toml_contents.as_table().unwrap().get("module").unwrap().clone());
  comb_build_contents.push_str(&comb_build_generator.to_string());
  comb_build_contents.push('\n');

  comb_build_generator.as_table_mut().unwrap().remove("module");
  comb_build_generator.as_table_mut().unwrap().insert("dependencies".to_string(), toml_contents.as_table().unwrap().get("dependencies").unwrap().clone());
  comb_build_contents.push_str(&comb_build_generator.to_string());
  comb_build_contents.push('\n');

  match comb_build.write_all(comb_build_contents.as_bytes()) {
    Err(_) => return Err(GenerateFailed::FileError("failed to write to comb.build".to_string())),
    Ok(_) => (),
  }

  Ok(())
}