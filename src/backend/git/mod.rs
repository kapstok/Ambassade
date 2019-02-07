pub mod status;
pub mod ignore;
mod exe;

pub fn to_shell<I>(args: &mut I) where I: Iterator<Item=String> {
    match args.next() {
        Some(ref arg) if arg == "rec" => match exe::exe_rec(args.collect()) {
            Ok(_) => {},
            Err(e) => super::normal(format!("Executing recursive git command failed. Details: {}", e))
        },
        Some(git) => {
            let path = match super::filesystem::get_current_project_root() {
                Some(result) => result,
                None => {
                    super::normal(format!("Executing git command failed. Details: {}", "Not in a project (sub)directory."));
                    return;
                }
            };

            let mut args: Vec<String> = args.collect();
            args.insert(0, git);

            match exe::exe(&path, args) {
                Ok(_) => {},
                Err(e) => super::normal(format!("Executing git command failed. Details: {}", e))
            }
        },
        None => {}
    }
}
