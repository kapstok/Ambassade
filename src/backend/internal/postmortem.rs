extern crate ambassade_debug as dbg;

use backend;

pub struct DefaultSubmitMethod {}

impl dbg::debug::SubmitMethod for DefaultSubmitMethod {
    fn submit(&self, _info: &dbg::debug::DebugInfo) -> bool {
        true
    }

    fn submission_succeeded(&self, info: &dbg::debug::DebugInfo) {
        backend::normal("");
        backend::normal("Please submit the information below to the repository or to a mirror:");
        backend::normal("https://pagure.io/Ambassade/new_issue");
        backend::normal("https://github.com/kapstok/Ambassade/issues/new");
        backend::normal("");
        backend::normal("");
        backend::normal(info);
    }
}
