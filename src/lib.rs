

pub mod list_io {
    pub fn get_list(file_name: &str) -> Result<Vec<String>, std::io::Error> {
        let file_text = match std::fs::read_to_string(file_name) {
            Ok(text) => text,
            Err(e) => return Err(e),
        };

        let mut list: Vec<String> = Vec::new();
        for title in file_text.lines() {
            list.push(String::from(title));
        }
        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::list_io;
    #[test]
    fn correct_file_contents() {
        assert_eq!(vec!["eawgf", "hgre", "ghfesd", "ds"],
         list_io::get_list("test.txt").unwrap());
    }

    #[test]
    fn nonexistant_file() {
        assert!(list_io::get_list("random").is_err());
    }
}