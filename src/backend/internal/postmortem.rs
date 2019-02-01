extern crate ambassade_debug as dbg;

pub struct DefaultSubmitMethod {}

impl dbg::debug::SubmitMethod for DefaultSubmitMethod {
    fn submit(&self, _info: &dbg::debug::DebugInfo) -> bool {
        true
    }

    fn submission_succeeded(&self, info: &dbg::debug::DebugInfo) {
        println!("");
        println!("Please submit the information below to the repository or to a mirror:");
        println!("https://pagure.io/Ambassade/new_issue");
        println!("https://github.com/kapstok/Ambassade/issues/new");
        println!("");
        println!("");
        println!("{}", info);
    }
}
