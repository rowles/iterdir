use std::fs;
use std::path::PathBuf;

pub struct IterDir {
    start_path: Option<PathBuf>,
    dir_stack: Vec<fs::ReadDir>,
}

impl IterDir {
    pub fn new(path: &PathBuf) -> IterDir {
        IterDir {
            start_path: Some(path.to_path_buf()),
            dir_stack: vec![],
        }
    }
}

impl Iterator for IterDir {
    type Item = fs::DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(start_path) = self.start_path.take() {
            let readdir = fs::read_dir(start_path).expect("Err: could not read directory");
            self.dir_stack.push(readdir);
        }

        while !self.dir_stack.is_empty() {
            let curr = self
                .dir_stack
                .last_mut()
                .expect("Err: stack is empty")
                .next();

            match curr {
                None => {
                    self.dir_stack.pop().expect("Err: could not pop stack");
                }
                Some(Ok(dent)) => {
                    let ftype = dent.file_type().expect("Err: could not get file type");

                    if ftype.is_dir() {
                        let readdir =
                            fs::read_dir(dent.path()).expect("Err: could not read directory");
                        self.dir_stack.push(readdir);
                    }

                    return Some(dent);
                }
                Some(Err(_)) => return None,
            }
        }

        None
    }
}
