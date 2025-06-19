use std::ffi::OsString;
use std::path::Path;


use regex::Regex;

static MEDIA_EXTENSIONS: [&str; 2] = [r"(?i)\.mp4$", r"(?i)\.mkv$"];

static TELEVION_PATTERNS: [&str; 4] = ["(?i)series", "(?i)complete", "(?i)(s|e)\\d{1,2,3}", "(?i)season"];

static TITLE_END_PATTERNS: [&str; 5] = ["^(19|20)\\d{2}", "(?i)series", "(?i)complete", "(?i)(s|e)\\d{1,2,3}", "(?i)season"];


/// Holds type of media. Television or Movie, if its television then we also store
/// the Season Number.
enum MediaType {
    Television(u32),
    Movie
}

pub struct Media<'a> {
    title: String,
    path: &'a Path,
    media_type: MediaType,
}

fn find_token(title: &str, pat: &str) -> Option<usize> {
    let re = Regex::new(pat).unwrap();
    re.find(title).map(| m | m.start())
}

fn is_television(file: &str) -> bool {
    for pat in TELEVION_PATTERNS {
        let re = Regex::new(pat).unwrap();
        if re.is_match(file) {
            return true;
        }
    }
    false
}

fn find_season_number(file: &str) -> Option<u32> {
    let re = Regex::new(r"(?i)s\d{1,3}").unwrap();
    re.find(file).map(
        |m| m.as_str()[1..].parse().expect("season matched but no number")
    )
}

pub fn is_media(path: &str) -> bool {

    for pat in MEDIA_EXTENSIONS {
        let re = Regex::new(pat).unwrap();
        if re.is_match(path) {
            return true;
        }
    }
    false
}

fn clean_file_name(file: &str) -> String {
    let re = Regex::new(r"(\.|\(|\))").unwrap();
    re.replace_all(file, " ").into_owned()
}

fn get_title_end_pos(file: &str) -> Option<usize> {
    let mut min: usize = 1000;
    for pat in TITLE_END_PATTERNS {
        let re = Regex::new(pat).unwrap();
        match re.find(file) {
            Some(m) => {
                if m.start() < min {
                    min = m.start();
                }
            },
            None => {continue;}
        }
    }
    if min != 1000 {
        return Some(min);
    }
    None
}

pub fn parse(path: &OsString) -> () {
    let path = Path::new(path);

    let file_name = match path.file_name() {
        Some(file) => file.to_str().unwrap(),
        None => return
    }; 

    if !is_media(file_name) {
        return
    };
    
    let directory = match path.parent().unwrap().file_name() {
        Some(file) => file.to_str().unwrap(),
        None => return
    };

    

}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_media() {
        let path = String::from("path/to/variable.mkv");
        let result = is_media(&path);
        assert!(result);
        let path = String::from("path/to/variable.mp4");
        let result = is_media(&path);
        assert!(result);
        let path = String::from("path/to/vari.mp4.hi");
        let result = is_media(&path);
        assert!(!result);
        let path = String::from("path/to/vari.png");
        let result = is_media(&path);
        assert!(!result);
    }

    #[test]
    fn test_find_season() {
        let file = String::from("myshows01");
        let result = find_season_number(&file).unwrap();
        assert_eq!(result, 1);
        let file = String::from("myshows01");
        let result = find_season_number(&file).unwrap();
        assert_eq!(result, 1);
        
        let file = String::from("myshows2");
        let result = find_season_number(&file).unwrap();
        assert_eq!(result, 2);
        let file = String::from("myshowS2");
        let result = find_season_number(&file).unwrap();
        assert_eq!(result, 2);
    }
}

