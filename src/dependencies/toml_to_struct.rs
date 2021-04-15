use toml::Value;

#[derive(Debug)]
pub enum DependencyLocation {
  LocalFS(String),
  Repository((String, String)),
}

#[derive(Debug)]
pub struct Dependency {
  pub name: String,
  pub version: (usize, usize, usize),
  pub location: DependencyLocation,
}

pub fn toml_table_to_structs(
  dependency_table: &toml::value::Table,
) -> Vec<Dependency> {
  let mut dependency_list = Vec::new();

  for (dependency_name, dependency_info) in dependency_table.iter() {
    let dependency_struct = toml_to_struct(dependency_name, dependency_info);

    dependency_list.push(dependency_struct);
  }

  dependency_list
}

fn toml_to_struct(
  dependency_name: &String,
  dependency_info: &Value,
) -> Dependency {
  let dependency_table = dependency_info.as_table().unwrap();


  let string_to_int_closure = |v: &str| v.parse::<usize>().unwrap();

  let mut version_iterator = if dependency_table.contains_key("version") {
    dependency_table.get("version").unwrap().as_str().unwrap().split(".").map(string_to_int_closure)
  } else {
    "0.0.0".split(".").map(string_to_int_closure)
  };
  let dependency_version = (version_iterator.next().unwrap(), version_iterator.next().unwrap(), version_iterator.next().unwrap());


  let dependency_location;
  if dependency_table.contains_key("source") {
    dependency_location = DependencyLocation::LocalFS(
      dependency_table.get("source").unwrap().as_str().unwrap().to_string(),
    )
  } else {
    dependency_location = DependencyLocation::Repository((
      dependency_table.get("repo").unwrap().as_str().unwrap().to_string(),
      dependency_table.get("repo_path").unwrap().as_str().unwrap().to_string(),
    ))
  }


  Dependency {
    name: dependency_name.clone(),
    version: dependency_version,
    location: dependency_location,
  }
}