use super::toml_to_struct::Dependency;
use super::toml_to_struct::DependencyLocation;
use std::path::Path;
use std::path::PathBuf;
use std::fs;
use fs_extra::dir;
use std::env;

#[derive(Debug)]
pub enum DependencyFail {
  FailedToGet(String),
  ModuleDoesNotExist(String),
  PathDoesNotExist(String),
  RepoDoesNotExist(String),
  BuildDirFail(String),
}

pub fn get_dependencies(
  dependencies: &Vec<Dependency>,
) -> Result<(), DependencyFail> {
  

  for dependency in dependencies.iter() {
    match fs::create_dir(&dependency.name) {
      Ok(_) => (),
      Err(_) => return Err(DependencyFail::BuildDirFail(format!("failed to create directory build/{}", &dependency.name)))
    }
    match &dependency.location {
      DependencyLocation::LocalFS(path) => {
        get_dependency_local_fs(dependency, &shellexpand::tilde(path))?;
      }
      DependencyLocation::Repository(repo) => {
        get_version(repo)?;
      }
    }
    
  }
  Ok(())
}

pub fn get_version(
  repo_location: &(String, String)
) -> Result<Option<String>, DependencyFail> {
  println!("Path {} in repo at {}", repo_location.1, repo_location.0);

  Ok(None)
}

pub fn get_dependency_local_fs(
  dep: &Dependency,
  path: &str,
) -> Result<(), DependencyFail> {
  let relative_path = if Path::new(path).has_root() {
    path.to_string()
  } else {
    ["../", path].join("")
  };

  if !Path::new(&relative_path).exists() {
    return Err(DependencyFail::PathDoesNotExist(format!(
      "path {} (module {}) does not exist",
      path,
      dep.name,
    )))
  }

  if ![&relative_path, &"comb.toml".to_string()].iter().collect::<PathBuf>().exists() {
    return Err(DependencyFail::ModuleDoesNotExist(format!(
      "module {} does not exist at {}",
      dep.name,
      path,
    )))
  }

  if copy_dir_contents(&relative_path, &dep.name).is_err() {
    return Err(DependencyFail::FailedToGet(format!(
      "failed to copy contents of dir '{}' into path 'build/{}'",
      path,
      dep.name,
    )))
  }

  Ok(())
}

fn copy_dir_contents(
  src_path: &str,
  dest_path: &str,
) -> Result<(), ()> {
  let mut items_to_copy = Vec::new();

  match fs::read_dir(src_path) {
    Ok(contents) => {
      for item in contents {
        match item {
          Ok(dir_item) => items_to_copy.push(dir_item.path()),
          Err(_) => return Err(()),
        }
      }
    }
    Err(_) => return Err(()),
  }

  match fs_extra::copy_items(&items_to_copy, dest_path, &dir::CopyOptions::new()) {
    Ok(_) => Ok(()),
    Err(_) => Err(()),
  }
}