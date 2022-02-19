use std::fmt;
use std::path::Path;

#[derive(Debug)]
pub struct PlayList {
    pub urls: Vec<String>,
    pub cur_index: usize,
}

impl PlayList {
    pub fn new(urls: Vec<String>) -> Self {
        Self { urls, cur_index: 0 }
    }
}

impl fmt::Display for PlayList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_str = "".to_owned();
        for (idx, url) in self.urls.iter().enumerate() {
            let indicator = if idx == self.cur_index {
                "*".to_owned()
            } else {
                (idx + 1).to_string()
            };
            let path = Path::new(&url)
                .file_name()
                .and_then(|p| p.to_str())
                .unwrap_or("unknown file");
            fmt_str.push_str(&format!("[{}] {}\n", indicator, path));
        }

        write!(f, "{}", fmt_str)
    }
}
