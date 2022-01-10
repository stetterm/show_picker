use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

pub mod list_io {

    pub struct IOObject<'a> {
        pub path: &'a str
    }

    impl <'a>IOObject<'a> {
        pub fn get_list(&self) -> Result<Vec<String>, std::io::Error> {
            let file_text = match std::fs::read_to_string(self.path) {
                Ok(text) => text,
                Err(e) => return Err(e),
            };

            let mut list: Vec<String> = Vec::new();
            for title in file_text.lines() {
                list.push(String::from(title));
            }
            Ok(list)
        }

        pub fn write_list(&self, choice: &str, titles: Vec<String>) 
          -> Result<(), std::io::Error> {
            let mut file = match std::fs::File::create(self.path) {
                Ok(f) => f,
                Err(e) => { return Err(e); }
            };
            for title in titles {
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::list_io;
    #[test]
    fn correct_file_contents() {
        let test = list_io::IOObject {
            path: "test.txt"
        };
        assert_eq!(vec!["eawgf", "hgre", "ghfesd", "ds"],
         test.get_list().unwrap());
    }

    #[test]
    fn nonexistant_file() {
        let test = list_io::IOObject {
            path: "random"
        };
        assert!(test.get_list().is_err());
    }
}