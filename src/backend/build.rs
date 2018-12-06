use std::io::Result;

pub fn build(config: String) -> Result<String> {
    println!("Building project..");

    match super::check::dep(config) {
        Ok(result) => println!("{}", result),
        Err(e) => return Err(e)
    }

    Ok(String::from("Build succeeded!"))
}
