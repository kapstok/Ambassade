use std::path::PathBuf;

pub fn init(mut dep_name: String) -> Result<(), String> {
    let mut config = match super::filesystem::get_dep_config_root() {
        Ok(path) => path,
        Err(e) => return Err(e.to_string())
    };

    dep_name.push_str(".json");
    config.push(dep_name);

    match super::config::init(&config) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

pub fn scan(dep_name: String) -> Result<PathBuf, String> {
    super::log(format!("Scanning for {}..", dep_name));

    let mut config_path = match super::filesystem::get_dep_config_root() {
        Ok(path) => path,
        Err(e) => return Err(e.to_string())
    };

    let mut config_name = dep_name.clone();
    config_name.push_str(".json");
    config_path.push(config_name);

    if config_path.is_file() {
        return Ok(config_path);
    }

    let project_path = match super::filesystem::get_current_project_root() {
        Some(mut path) => {
            if String::from(path.file_name().unwrap().to_str().unwrap()) == dep_name {
                super::log("Project name equals dep name. Taking project's configfile...");
                path.push("ambassade.json");
                return Ok(path);
            } else {
                path
            }
        },
        None => return Err(String::from("Not in a project (sub)directory."))
    };

    match super::filesystem::get_current_dep_root() {
        Ok(mut path) => {
            path.push(dep_name.clone());
            path.push("ambassade.json");
            match super::config::get_json(&path) { // Inconventient, should be changed later
                Ok(_) => Ok(path),
                _ => {
                    match super::config::get_json_from_dir(project_path) {
                        Ok(json) => match super::dep::check(json) {
                            Ok(result) => {
                                super::log(format!("{}", result));
                                return scan(dep_name);
                            },
                            Err(e) => return Err(e)
                        },
                        Err(e) => return Err(e)
                    }
                }
            }
        },
        Err(e) => Err(e.to_string())
    }
}
