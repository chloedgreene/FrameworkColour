use std::process::Command;

fn main() {
    

    //start building ectools


 
    Command::new("sh").args(&["buildectools.sh"]).status().unwrap();

     
    println!("cargo:rerun-if-changed=framework-ec/util/ectool.c");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=buildectools.sh");
    println!("cargo:rerun-if-changed=Cargo.lock");

}