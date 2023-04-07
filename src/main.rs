use std::{
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use log::{error, info};

fn main() {
    env_logger::init();

    let mut runner = Runner::new();
    ctrlc::set_handler(|| info!("receiving exit signal")).expect("setting Ctrl-C handler failed");
    runner.close()
}

struct Runner {
    runner: Option<thread::JoinHandle<()>>,
    done: Arc<AtomicBool>,
    ip: &'static str,
}

impl Runner {
    fn new() -> Self {
        let done = Arc::new(AtomicBool::new(false));
        let t_done = done.clone();

        let mut runner = Runner {
            ip: "",
            runner: None,
            done,
        };

        runner.runner = Some(thread::spawn(move || {
            while !t_done.load(Ordering::SeqCst) {
                if let Err(e) = run(runner.ip) {
                    error!("{}", e);
                }
            }
        }));

        runner
    }

    fn close(&mut self) {
        self.done.store(true, Ordering::SeqCst);
        self.runner
            .take()
            .expect("runner close failed")
            .join()
            .expect("runner close failed");
    }
}

fn run(org_ip: &str) -> Result<(), Box<dyn Error>> {
    let response = ureq::get("https://api.ipify.org").call()?;
    let ip = response.into_string()?;
    if !org_ip.is_empty() && org_ip != ip {
        // TODO: calling the google API to update DNS record
    }
    Ok(())
}
