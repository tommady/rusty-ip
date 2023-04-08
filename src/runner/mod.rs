mod google;

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{sync_channel, SyncSender},
        Arc, Condvar, Mutex,
    },
    thread,
    time::Duration,
};

use crate::config::Runner as RunnerType;

use log::error;

pub(crate) struct Runner {
    runners: Vec<Option<thread::JoinHandle<()>>>,
    done: Arc<AtomicBool>,
    done_notify: Arc<(Mutex<String>, Condvar)>,
    done_sleep: SyncSender<bool>,
}

impl Runner {
    pub(crate) fn new(cfg: &crate::config::Config) -> Self {
        let done = Arc::new(AtomicBool::new(false));
        let mut runners = Vec::with_capacity(cfg.runners.len() + 1);
        let original_ip = String::new();
        let rx_notify = Arc::new((Mutex::new(original_ip), Condvar::new()));
        let tx_notify = rx_notify.clone();
        let done_notify = tx_notify.clone();
        let (done_sleep, sleep) = sync_channel::<bool>(1);

        // spawn a ticker crawler to get the current ip
        let freq = Duration::from_secs(cfg.check_interval_sec);
        let tx_done = done.clone();
        runners.push(Some(thread::spawn(move || {
            while !tx_done.load(Ordering::SeqCst) {
                fn get_current_ip() -> crate::Result<String> {
                    Ok(ureq::get("https://api.ipify.org").call()?.into_string()?)
                }

                match get_current_ip() {
                    Ok(new_ip) => {
                        let (lock, cvar) = &*tx_notify;
                        // should never be poisoned
                        let mut original_ip = lock.lock().unwrap();
                        if original_ip.is_empty() || *original_ip != new_ip {
                            *original_ip = new_ip;
                            cvar.notify_all();
                        }
                    }
                    Err(e) => {
                        error!("get_current_ip failed: {}", e);
                    }
                }

                let _ = sleep.recv_timeout(freq);
            }
        })));

        // spawn actual runners
        for runner_cfg in &cfg.runners {
            let rx_done = done.clone();
            let rx_notify = rx_notify.clone();
            let runner = match runner_cfg {
                RunnerType::Google {
                    hostname,
                    username,
                    password,
                } => google::Google::new(username, password, hostname),
            };

            runners.push(Some(thread::spawn(move || {
                while !rx_done.load(Ordering::SeqCst) {
                    let (lock, cvar) = &*rx_notify;
                    let ip = cvar.wait(lock.lock().unwrap()).unwrap();

                    if let Err(e) = runner.run(&ip) {
                        error!("{}", e);
                    }
                }
            })))
        }

        Runner {
            runners,
            done,
            done_notify,
            done_sleep,
        }
    }

    pub(crate) fn close(&mut self) {
        self.done.store(true, Ordering::SeqCst);
        let _ = self.done_sleep.send(true);

        let (_, cvar) = &*self.done_notify;
        cvar.notify_all();

        for runner in self.runners.iter_mut() {
            if let Some(w) = runner.take() {
                let _ = w.join();
            }
        }
    }
}
