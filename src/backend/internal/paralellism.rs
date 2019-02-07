use backend;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::result::Result;
use std::path::PathBuf;
use std::time::Duration;

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
                backend::log(format!("Could not add job: {}", "Threadhandler already started."));
                false
            },
            false => {
                self.jobs.push((backend::fetch::fetch, dep_name, dep_path, command));
                true
            }
        }
    }

    pub fn start(&mut self) {
        if self.triggered {
            backend::log(format!("{}", "Threadhandler already started."));
            return
        }

        self.triggered = true;
        let mut handles = vec![];
        let running_jobs = Arc::new(AtomicUsize::new(0));


        for job in self.jobs.clone() {
            backend::log(format!("Thread: {:?}", &job));
            running_jobs.fetch_add(1, Ordering::Release);
            let running_jobs = Arc::clone(&running_jobs);

            let handle = thread::spawn(move|| {
                match &job.0(job.1.clone(), job.2.clone(), job.3.clone()) {
                    Ok(msg) => backend::normal(msg),
                    Err(e) => backend::normal(format!("Thread {:?} paniced. Details: {}", &job, e))
                }

                running_jobs.fetch_sub(1, Ordering::Relaxed);
            });

            handles.push(handle);
        }

        backend::log("All scheduled jobs are running! Waiting for job to finish..");

        while running_jobs.load(Ordering::Relaxed) != 0 {
            thread::sleep(Duration::from_secs(3));
        }

        for handle in handles {
            backend::log("Join handle..");
            handle.join().unwrap();
            backend::log("Handle joined.");
        }
    }
}

impl Drop for Threadhandler {
    fn drop(&mut self) {
        backend::log("Threadhandler dropped!");
    }
}
