use std::path::PathBuf;
use std::fs::{File, remove_file};

pub struct Lock {
    path: PathBuf,
}

impl Lock {
    pub fn acquire(path: &str) -> Self {
        let p = PathBuf::from(path);
        let _ = File::create(&p);
        Self { path: p }
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        let _ = remove_file(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lock() {
        let path = "test.lock";
        {
            let _lock = Lock::acquire(path);
            assert!(std::path::Path::new(path).exists());
        }
        assert!(!std::path::Path::new(path).exists());
    }
}
