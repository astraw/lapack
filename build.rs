use std::env;

fn main() {
  let target = env::var("TARGET").unwrap();

  if target.contains("darwin") {
    // According to http://www.lapk.org/gfortran/gfortran.php?OS=7
    println!("cargo:rustc-link-search=/usr/local/gfortran/lib");
  }
}
