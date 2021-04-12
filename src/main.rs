use comb::toml::build_toml_generation;

fn main() {
  let build_file_input = "comb.toml";
  let build_file_output = "comb.build.toml";

  match build_toml_generation::generate_build_toml(build_file_input, build_file_output, false) {
    Err(_) => return,
    _ => (),
  }
}


