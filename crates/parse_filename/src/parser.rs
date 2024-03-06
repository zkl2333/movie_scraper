use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref RE_YEAR: Regex = Regex::new(r"\d{4}").unwrap();
    static ref RE_SUBTITLE: Regex = Regex::new(r"(中英特效字幕|简繁中字)").unwrap();
    static ref RE_DEPTH: Regex = Regex::new(r"\d+bit").unwrap();
}

// 统计并找出最常见的分隔符
fn most_common_separator(filename: &str) -> char {
    let mut counter = HashMap::new();
    for c in filename.chars() {
        if c.is_ascii_punctuation() || c.is_whitespace() {
            *counter.entry(c).or_insert(0) += 1;
        }
    }

    counter
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(c, _)| c)
        .unwrap_or('_')
}

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
    media_audio_channel: HashMap<String, String>,
    media_codec: HashMap<String, String>,
    resolution: HashMap<String, String>,
    release_team: HashMap<String, String>,
}

pub struct Parser {
    config: ParserData,
    groups: Vec<String>,
}

impl Parser {
    pub fn new(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config = read_media_stream()?;

        Ok(Parser {
            config,
            groups: filename
                .split(most_common_separator(filename))
                .map(|s| s.to_string())
                .collect(),
        })
    }

    fn match_attribute<'a>(&self, data: &'a HashMap<String, String>) -> Option<(String, String)> {
        let mut result: Option<(String, String)> = None;
        let groups = &self.groups;

        for group in groups {
            for (k, v) in data.iter() {
                let re = Regex::new(format!(r"(?i){}", k).as_str()).unwrap();
                match re.find(group.as_str()) {
                    Some(m) => {
                        // 如果找到匹配项，判断是否是最长的匹配项
                        if let Some((ref old, _)) = result {
                            if m.as_str().len() > old.len() {
                                result = Some((group.to_string(), v.to_string()));
                            }
                        } else {
                            result = Some((group.to_string(), v.to_string()));
                        }
                    }
                    None => continue,
                }
            }
            if result.is_some() {
                break;
            }
        }
        result
    }

    // 解析年份
    pub fn match_year(&self) -> Option<(String, String)> {
        self.groups
            .iter()
            .find(|s| RE_YEAR.is_match(s))
            .map(|s| (s.to_string(), s.to_string()))
    }

    // 解析来源
    pub fn match_source(&self) -> Option<(String, String)> {
        self.match_attribute(&self.config.media_source)
    }

    // 解析分辨率
    pub fn match_resolution(&self) -> Option<(String, String)> {
        self.match_attribute(&self.config.resolution)
    }

    // 解析编码
    pub fn match_encoding(&self) -> Option<(String, String)> {
        self.match_attribute(&self.config.media_codec)
    }

    // 解析发布组
    pub fn match_release_team(&self) -> Option<(String, String)> {
        self.match_attribute(&self.config.release_team)
    }

    // 解析音频编码
    pub fn match_audio(&self) -> Option<(String, String)> {
        self.match_attribute(&self.config.media_audio)
    }

    // 解析声道
    pub fn match_channel(&self) -> Option<(String, String)> {
        self.match_attribute(&self.config.media_audio_channel)
    }

    // 解析字幕
    pub fn match_subtitle(&self) -> Option<(String, String)> {
        self.groups
            .iter()
            .find(|s| RE_SUBTITLE.is_match(s))
            .map(|s| (s.to_string(), s.to_string()))
    }

    // 解析深度
    pub fn match_depth(&self) -> Option<(String, String)> {
        self.groups
            .iter()
            .find(|s| RE_DEPTH.is_match(s))
            .map(|s| (s.to_string(), s.to_string()))
    }
}
