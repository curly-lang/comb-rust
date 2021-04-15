use std::env;
use std::fs;
use toml::Value;

use comb::toml::toml_parser;
use comb::toml::build_toml_generation;
use comb::dependencies::get;
use comb::dependencies::get::DependencyFail;
use comb::dependencies::toml_to_struct;

fn main() -> Result<(), ()> {
  let mut args = env::args();
  let name = args.next().unwrap();

  let build_file_input = "comb.toml";
  let build_file_output = "comb.build.toml";

  if args.len() == 0 {
    println!(
      "usage:\n{} update\n{} build",
      &name,
      &name
    );
    Ok(())
  } else {
    match args.next().unwrap().as_str() {
      "update" => update(build_file_input, build_file_output, true),
      "build"  => build(build_file_input, build_file_output),
      _ => Err(())
    }
  }
}

fn update(
  file_in: &str,
  file_out: &str,
  force_update: bool,
) -> Result<(), ()> {
  return build_toml_generation::generate_build_toml(file_in, file_out, force_update);
}

fn make_and_cd_build() -> Result<(), ()> {
  let _ = fs::remove_dir_all("build");
  match fs::create_dir("build") {
    Ok(_) => (),
    Err(_) => {
      println!("failed to create build directory");
      return Err(())
    }
  }
  match env::set_current_dir("build") {
    Ok(_) => (),
    Err(_) => {
      println!("failed to cd into the build directory");
      return Err(())
    }
  }
  Ok(())
}

fn compile_dependencies(
  build_toml: &Value,
) -> Result<(), ()> {
  let dependency_table = build_toml.as_table().unwrap().get("dependencies").unwrap().as_table().unwrap();
  let dependency_list = toml_to_struct::toml_table_to_structs(dependency_table);

  match get::get_dependencies(&dependency_list) {
    Ok(_) => (),
    Err(e) => {
      match e {
        DependencyFail::FailedToGet(err)        => println!("Failed to get dependency:\n{}", err),
        DependencyFail::ModuleDoesNotExist(err) => println!("Module does not exist:\n{}", err),
        DependencyFail::PathDoesNotExist(err)   => println!("Path does not exist:\n{}", err),
        DependencyFail::RepoDoesNotExist(err)   => println!("Repository does not exist:\n{}", err),
        DependencyFail::BuildDirFail(err)       => println!("Failed to create directory:\n{}", err),
      }
      return Err(())
    }
  }
  println!("{:?}", dependency_list);
  Ok(())
}

fn build(
  build_file_in: &str,
  build_file_out: &str,

) -> Result<(), ()> {
  match update(build_file_in, build_file_out, false) {
    Ok(_) => (),
    Err(_) => return Err(()),
  }

  let build_toml = toml_parser::parse_toml(build_file_out).unwrap();

  make_and_cd_build()?;

  match compile_dependencies(&build_toml) {
    Ok(_) => (),
    Err(_) => return Err(()),
  }

  Ok(())
}