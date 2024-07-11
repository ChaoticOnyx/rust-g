use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        RwLock,
    },
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
struct Webhook {
    pub url: String,
    pub message: String,
}

static WEBHOOK: RwLock<Option<Webhook>> = RwLock::new(None);
static STOP: AtomicBool = AtomicBool::new(false);
static TIMEOUT: RwLock<Option<Instant>> = RwLock::new(None);

fn update(timeout: &str) -> Result<(), String> {
    let timeout = str::parse::<u64>(&timeout).map_err(|err| err.to_string())?;
    let timeout = Duration::from_secs(timeout);
    let timeout = Instant::now() + timeout;

    let mut lock = TIMEOUT.write().unwrap();

    lock.replace(timeout);

    Ok(())
}

fn start(timeout: &str, webhook_url: &str, message: &str) -> Result<(), String> {
    update(timeout)?;

    if !webhook_url.is_empty() && !message.is_empty() {
        WEBHOOK.write().unwrap().replace(Webhook {
            url: webhook_url.to_string(),
            message: message.to_string(),
        });
    }

    STOP.store(false, Ordering::Relaxed);

    std::thread::spawn(|| {
        while !STOP.load(Ordering::Relaxed) {
            let timeout = { TIMEOUT.read().unwrap().unwrap() };

            if Instant::now() > timeout {
                if let Some(webhook) = WEBHOOK.read().unwrap().clone() {
                    let client = reqwest::blocking::Client::new();
                    let request = client
                        .post(webhook.url)
                        .json(&HashMap::from([("content", webhook.message)]))
                        .build()
                        .unwrap();

                    client.execute(request).ok();
                }

                unsafe {
                    #[cfg(not(target_os = "windows"))]
                    libc::raise(libc::SIGUSR2);
                    #[cfg(target_os = "windows")]
                    libc::raise(libc::SIGSEGV);
                }
            }

            std::thread::sleep(Duration::from_secs(1));
        }
    });

    Ok(())
}

fn stop() {
    STOP.swap(true, Ordering::Relaxed);
}

byond_fn!(fn wd_update(timeout) {
    match update(timeout) {
        Ok(_) => Some(String::new()),
        Err(err) => Some(err),
    }
});

byond_fn!(
    fn wd_start(timeout, webhook_url, message) {
        match start(timeout, webhook_url, message) {
            Ok(_) => Some(String::new()),
            Err(err) => Some(err),
        }
    }
);

byond_fn!(
    fn wd_stop() {
        stop();

        Some(String::new())
    }
);
