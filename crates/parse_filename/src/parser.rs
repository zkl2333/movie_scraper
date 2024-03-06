use std::collections::HashMap;

// parser.rs
use regex::Regex;
use serde::{Deserialize, Serialize};

fn read_media_stream() -> Result<ParserData, Box<dyn std::error::Error>> {
    let file = std::fs::File::open("media_stream.json")?;
    let reader = std::io::BufReader::new(file);
    let media_stream: ParserData = serde_json::from_reader(reader)?;
    Ok(media_stream)
}

#[derive(Debug, Serialize, Deserialize)]
struct ParserData {
    media_source: HashMap<String, String>,
    media_audio: HashMap<String, String>,
    media_codec: HashMap<String, String>,
    resolution: HashMap<String, String>,
    release_team: HashMap<String, String>,
}

pub struct Parser {
    config: ParserData,
}

impl Parser {
    pub fn new() -> Self {
        let config = read_media_stream().unwrap();
        Parser { config }
    }

    // 解析年份
    pub fn match_yaer(&self, filename: &str) -> Option<String> {
        let re_year = Regex::new(r"\d{4}").unwrap();
        re_year.find(filename).map(|m| m.as_str().to_string())
    }

    // 解析来源
    pub fn match_source(&self, filename: &str) -> Option<(String, String)> {
        let media_source = &self.config.media_source;
        for (k, v) in media_source {
            // 大小写不敏感
            // let re = Regex::new(&k).unwrap();
            let re = Regex::new(format!("(?i){}", k).as_str()).unwrap();
            if re.is_match(filename) {
                return Some((k.to_string(), v.to_string()));
            }
        }
        None
    }

    // 解析分辨率
    pub fn match_resolution(&self, filename: &str) -> Option<(String, String)> {
        let resolution = &self.config.resolution;
        for (k, v) in resolution {
            let re = Regex::new(k).unwrap();
            if re.is_match(filename) {
                return Some((k.to_string(), v.to_string()));
            }
        }
        None
    }

    // 解析编码
    pub fn match_encoding(&self, filename: &str) -> Option<(String, String)> {
        let media_codec = &self.config.media_codec;
        for (k, v) in media_codec {
            let re = Regex::new(k).unwrap();
            if re.is_match(filename) {
                return Some((k.to_string(), v.to_string()));
            }
        }
        None
    }

    // 解析发布组
    pub fn match_release_team(&self, filename: &str) -> Option<(String, String)> {
        let release_team = &self.config.release_team;
        for (k, v) in release_team {
            let re = Regex::new(k).unwrap();
            if re.is_match(filename) {
                return Some((k.to_string(), v.to_string()));
            }
        }
        None
    }

    // 解析音频
    pub fn match_audio(&self, filename: &str) -> Option<(String, String)> {
        let media_audio = &self.config.media_audio;
        for (k, v) in media_audio {
            let re = Regex::new(k).unwrap();
            if re.is_match(filename) {
                return Some((k.to_string(), v.to_string()));
            }
        }
        None
    }

    // 解析字幕
    pub fn match_subtitle(&self, filename: &str) -> Option<String> {
        let re_subtitle = Regex::new(r"(中英特效字幕|简繁中字)").unwrap();
        re_subtitle.find(filename).map(|m| m.as_str().to_string())
    }

    // 解析深度
    pub fn match_depth(&self, filename: &str) -> Option<String> {
        let re_depth = Regex::new(r"10bit").unwrap();
        re_depth.find(filename).map(|m| m.as_str().to_string())
    }
}
