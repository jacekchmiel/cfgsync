use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use cfgsync::config::Config;
use cfgsync::{CfgSync, Result};
use tempdir::TempDir;

struct TestBed {
    home: TempDir,
    config: Config,
}

impl TestBed {
    pub fn new() -> TestBed {
        return TestBed {
            home: TempDir::new("cfgsync_integration").expect("Failed to ceate temporary directory"),
            config: Config::default(),
        };
    }

    pub fn create_file(&self, file: &str, contents: &str) {
        let full_path = self.home.path().join(file);
        let mut file = File::create(full_path).expect("Failed to create file");
        file.write_all(contents.as_bytes())
            .expect("Failed to write file");
    }

    pub fn track(&mut self, file: &str) {
        self.config.syncset.insert(file.into());
    }

    pub fn cfg_file_path(&self) -> PathBuf {
        self.home.path().join("cfgsync.conf")
    }

    pub fn cfgsync(&self) -> CfgSync {
        CfgSync::new(
            Config::load_with_path(&self.cfg_file_path()).expect("Failed to initialize config"),
        )
    }
}

#[test]
fn can_track_file() -> Result<()> {
    let test = TestBed::new();
    let mut cfgsync = test.cfgsync();
    cfgsync.add("config.ini")?;

    let tracked = dbg!(cfgsync).list();
    assert!(dbg!(tracked).len() == 1);

    Ok(())
}

#[test]
fn tracked_files_are_prefixed_with_home() -> Result<()> {
    let test = TestBed::new();
    let mut cfgsync = test.cfgsync();
    cfgsync.add("config.ini")?;
    cfgsync.add("~/config2.ini")?;

    let tracked = dbg!(cfgsync).list();
    assert!(tracked.contains(&"~/config.ini".to_string()));
    assert!(tracked.contains(&"~/config2.ini".to_string()));

    Ok(())
}
