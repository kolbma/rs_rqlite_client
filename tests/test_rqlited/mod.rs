/// `test_rqlited`
use std::{
    fs::OpenOptions,
    io::Write,
    process::{Child, Command},
    sync::{
        atomic::{AtomicBool, AtomicU8, Ordering},
        RwLock,
    },
    time::Duration,
};

use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref TEST_RQLITED_DB: TestRqlited = TestRqlited::new();
}

pub(crate) struct TestRqlited {
    count: AtomicU8,
    is_rqlited_start: bool,
    is_started: AtomicBool,
    rqlited: RwLock<Option<Child>>,
}

impl TestRqlited {
    pub(crate) fn new() -> Self {
        let is_rqlited_start = !["0", "off", "no"].contains(
            &std::env::var("RQLITED_TESTS_START")
                .unwrap_or_default()
                .trim()
                .to_lowercase()
                .as_str(),
        );

        let rqlited = if is_rqlited_start {
            Some(Self::start())
        } else {
            None
        };
        let rqlited = RwLock::new(rqlited);

        Self {
            count: AtomicU8::default(),
            is_rqlited_start,
            is_started: AtomicBool::default(),
            rqlited,
        }
    }

    pub(crate) fn run_test<T>(&self, test: T)
    where
        T: FnOnce() + std::panic::UnwindSafe,
    {
        self.tearup();

        let result = std::panic::catch_unwind(test);

        self.teardown();

        assert!(result.is_ok());
    }

    fn start() -> Child {
        let data_dir = env!("OUT_DIR").to_string() + "/rqlite_data";

        let is_redirect_output = !["0", "off", "no"].contains(
            &std::env::var("RQLITED_REDIRECT_OUTPUT")
                .unwrap_or_default()
                .trim()
                .to_lowercase()
                .as_str(),
        );

        if is_redirect_output {
            let stdout_file = env!("OUT_DIR").to_string() + "/rqlited_stdout.log";
            let stdout = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&stdout_file)
                .unwrap();
            let _ = writeln!(std::io::stdout(), "rqlited stdout: {stdout_file}");

            let stderr_file = env!("OUT_DIR").to_string() + "/rqlited_stderr.log";
            let stderr = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&stderr_file)
                .unwrap();
            let _ = writeln!(std::io::stdout(), "rqlited stderr: {stderr_file}");

            Command::new(String::from("./rqlite/") + std::env::consts::ARCH + "/rqlite/rqlited")
                .arg(&data_dir)
                .stdout(stdout)
                .stderr(stderr)
                .spawn()
                .expect("rqlited spawned")
        } else {
            Command::new(String::from("./rqlite/") + std::env::consts::ARCH + "/rqlite/rqlited")
                .arg(&data_dir)
                .spawn()
                .expect("rqlited spawned")
        }
    }

    #[inline]
    fn stop(&self) -> std::io::Result<()> {
        if let Ok(mut rqlited) = self.rqlited.write() {
            if let Some(mut child) = rqlited.take() {
                let result = child.kill();
                self.is_started.store(false, Ordering::SeqCst);
                std::thread::sleep(Duration::from_millis(250));
                return result;
            }
        }
        Ok(())
    }

    #[inline]
    fn tearup(&self) {
        if self.is_rqlited_start {
            let _ = self.count.fetch_add(1, Ordering::SeqCst);
            if let Ok(mut rqlited) = self.rqlited.write() {
                if rqlited.is_none() {
                    *rqlited = Some(Self::start());
                    self.is_started.store(false, Ordering::SeqCst);
                }
                if !self.is_started.load(Ordering::Relaxed) {
                    std::thread::sleep(Duration::from_millis(3000));
                    self.is_started.store(true, Ordering::Relaxed);
                }
            }
        }
    }

    fn teardown(&self) {
        if self.count.fetch_sub(1, Ordering::SeqCst) == 1 {
            self.stop().expect("rqlited stopped");
        }
    }
}

impl Default for TestRqlited {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TestRqlited {
    fn drop(&mut self) {
        self.stop().expect("rqlited stopped");
    }
}
