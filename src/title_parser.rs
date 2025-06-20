#![allow(dead_code)]
#![allow(unused_variables)]
use std::path::Path;


use regex::Regex;

static MEDIA_EXTENSIONS: [&str; 2] = [r"(?i)\.mp4$", r"(?i)\.mkv$"];

static TELEVION_PATTERNS: [&str; 4] = ["(?i)series", "(?i)complete", "(?i)(s|e)\\d{1,3}", "(?i)season"];

static TITLE_END_PATTERNS: [&str; 5] = ["(19|20)\\d{2}", "(?i)series", "(?i)complete", "(?i)(s|e)\\d{1,3}", "(?i)season"];


/// Holds type of media. Television or Movie, if its television then we also store
/// the Season Number.
#[derive(Debug)]
#[derive(PartialEq)]
enum MediaType {
    Television(u32),
    Movie
}

#[derive(Debug)]
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

fn get_season_number(file: &str) -> Option<u32> {
    let re = Regex::new(r"(?i)s\d{1,3}").unwrap();
    re.find(file).map(
        |m| m.as_str()[1..].parse().expect("season matched but no number")
    )
}


fn get_movie_year(file: &str) -> Option<&str> {
    let re = Regex::new(r"(19|20)\d{2}").unwrap();
    re.find(file).map(
        |m| m.as_str()
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
    let result = re.replace_all(file, " ").into_owned();
    result.trim().to_owned()
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

pub fn parse(path: &str) -> Result<Media, &'static str> {
    let path = Path::new(path);

    let file_name = match path.file_name() {
        Some(file) => file.to_str().unwrap(),
        None => return Err("couldn't get the file name")
    }; 

    if !is_media(file_name) {
        return Err("this is not a media file")
    };

    let directory = match path.parent().unwrap().file_name() {
        Some(file) => file.to_str().unwrap(),
        None => return Err("Couldn't retrieve directory name")
    };

    // get the title string either from the directory or the file name.
    // If we can't get it from either return
    let title_string = match get_title_end_pos(directory) {
        Some(end_pos) => {
            &directory[..end_pos]
        },
        None => {
            let Some(end_pos) = get_title_end_pos(file_name) else {
                return Err("couldn't find ending to title");
            };
            &file_name[..end_pos]
        }
    };
    let mut title_string = clean_file_name(title_string);

    if is_television(file_name) {
        let season_number = get_season_number(file_name).unwrap();
        return Ok(Media{
            media_type: MediaType::Television(season_number),
            path,
            title: title_string,
        })
    }

    let year = get_movie_year(file_name).unwrap();
    title_string.push_str(&format!(" ({year})"));
    Ok(Media{
        media_type: MediaType::Movie,
        path,
        title: title_string
    })
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
    fn test_get_season() {
        let file = String::from("myshows01");
        let result = get_season_number(&file).unwrap();
        assert_eq!(result, 1);
        let file = String::from("myshows01");
        let result = get_season_number(&file).unwrap();
        assert_eq!(result, 1);
        
        let file = String::from("myshows2");
        let result = get_season_number(&file).unwrap();
        assert_eq!(result, 2);
        let file = String::from("myshowS2");
        let result = get_season_number(&file).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_parse() {
        let file = String::from("The.Super.Long.Television.S02E01.1080p.HEVC.x265/The.Super.Long.Television.S02E01.1080p.HEVC.x265.mkv");
        let result = parse(&file);
        match result {
            Ok(result)  => {
                assert_eq!(result.title, "The Super Long Television");
                assert_eq!(result.media_type, MediaType::Television(2));
            }
            Err(_e) => {
                panic!()
            }
        }

        let file = String::from("Nice.Movie.2019.1080p.BluRay.x265/Nice.Movie.2019.1080p.BluRay.x265.mp4");
        let result = parse(&file);
        match result {
            Ok(result)  => {
                assert_eq!(result.title, "Nice Movie (2019)");
                assert_eq!(result.media_type, MediaType::Movie);
            }
            Err(_e) => {
                panic!()
            }
        }

    }
}

