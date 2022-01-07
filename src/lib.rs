use std::fs;

pub mod list_io {
    pub fn get_list(file_name: &str) -> Vec<&str> {

        let contents = std::fs::read_to_string(file_name)
            .expect("Could not find specified file");

        let mut titles: Vec<&str> = Vec::new();
        
        vec!["1, 2, 3"]
    }
}