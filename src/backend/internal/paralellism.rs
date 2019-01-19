use backend;
use std::thread;
use std::result::Result;
use std::path::PathBuf;

type FetchPtr = fn(String, PathBuf, String) -> Result<String, String>;

pub struct Threadhandler {
    jobs: Vec<(FetchPtr, String, PathBuf, String)>,
    triggered: bool
}

impl Threadhandler {
    pub fn new() -> Threadhandler {
        Threadhandler {
            jobs: Vec::new(),
            triggered: false
        }
    }

    pub fn add(&mut self, dep_name: String, command: String) -> bool {
        let dep_path = match backend::filesystem::get_current_dep_root() {
            Ok(mut path) => {
                path.push(&dep_name);
                path
            },
            _ => return false
        };

        match self.triggered {
            true => {
                println!("Could not add job: {}", "Threadhandler already started.");
                false
            },
            false => {
                self.jobs.push((backend::fetch::fetch, dep_name, dep_path, command));
                true
            }
        }
    }

    pub fn start(&mut self) {
        let mut handles = vec![];

        if self.triggered {
            println!("{}", "Threadhandler already started.");
            return
        }

        self.triggered = true;

        for job in self.jobs.clone() {
            println!("Thread: {:?}", &job);
            let handle = thread::spawn(|| job.0(job.1, job.2, job.3));
            handles.push(handle);
        }

        println!("All scheduled jobs are running! Waiting for job to finish..");

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

impl Drop for Threadhandler {
    fn drop(&mut self) {
        println!("Threadhandler dropped!");
    }
}
