use toml::Value;

pub enum CompleteError {
  NotEnoughInfo(String),
  BadTomlType(String),
  DependencyFail(String),
}

pub fn complete_module(
  toml_contents: &mut Value
) -> Result<(), CompleteError> {
  let mut full_module_attributes: std::collections::HashMap<&str, fn(toml::value::Table) -> Value> = std::collections::HashMap::new();
  
  full_module_attributes.insert("name",          |t| t.get("name").unwrap_or(&Value::String("Main".to_string())).clone());
  full_module_attributes.insert("version",       |_| Value::String("0.0.1".to_string()));
  full_module_attributes.insert("curly_version", |_| Value::String("newest".to_string()));
  full_module_attributes.insert("links",         |_| Value::String("".to_string()));
  full_module_attributes.insert("binary_name",   |t| t.get("name").unwrap().clone());
  full_module_attributes.insert("C_FFI_files",   |_| Value::String("".to_string()));

  //{repo = "https://github.com/curly-lang/curstdlib", path = "io/fs"}
  
  let module_section;

  match toml_contents.get_mut("module") {
    Some(module) => {
      match module {
        Value::Table(_) => module_section = module.as_table_mut().unwrap(),
        _ => return Err(CompleteError::NotEnoughInfo("'module' is not a section or a table".to_string()))
      }
    }
    None => return Err(CompleteError::NotEnoughInfo("no 'module' section exists".to_string())),
  }

  for (module_attribute, attribute_gen) in full_module_attributes.iter() {
    if !module_section.contains_key(&module_attribute.to_string()) {
      module_section.insert(module_attribute.to_string(), attribute_gen(module_section.clone()));
    } else {
      match module_section.get(&module_attribute.to_string()).unwrap() {
        Value::String(_) => (),
        _ => return Err(CompleteError::BadTomlType(format!("section 'module', variable '{}' is not of type String", module_attribute))),
      }
    }
  }
  
  if !module_section.contains_key("authors") {
    module_section.insert("authors".to_string(), Value::Array(Vec::new()));
  } else {
    match module_section.get("authors").unwrap() {
      Value::Array(_) => (),
      _ => return Err(CompleteError::BadTomlType("section 'module', variable 'authors' is not of type Array".to_string())),
    }
  }


  if !module_section.contains_key("location") {
    let mut table_to_insert = toml::value::Map::new();
    table_to_insert.insert("path".to_string(), Value::String(".".to_string()));

    module_section.insert("location".to_string(), Value::String(".".to_string()));
  } else {
    match module_section.get("location").unwrap() {
      Value::String(path) => {
        let mut table_to_insert = toml::value::Map::new();
        table_to_insert.insert("path".to_string(), Value::String(path.clone()));

        module_section.insert("location".to_string(), Value::Table(table_to_insert));
      }
      Value::Table(location_table) => {
        let mut table_to_insert = location_table.clone();

        if !location_table.contains_key("path") {
          table_to_insert.insert("path".to_string(), Value::String(".".to_string()));
        }

        module_section.insert("location".to_string(), Value::Table(table_to_insert));
      }
      _ => {
        let error_message = "section 'module', variable 'location' is not of type Table or String".to_string();
        return Err(CompleteError::BadTomlType(error_message));
      }
    }
  }

  Ok(())
}