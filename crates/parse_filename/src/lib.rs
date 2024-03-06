mod parser;
use crate::parser::Parser;

macro_rules! match_parser {
    ($parser:expr, $method:ident) => {
        $parser
            .$method()
            .unwrap_or_else(|| ("".to_string(), "".to_string()))
    };
}

#[derive(Debug)]
pub struct MediaInfo {
    title: String,
    year: String,
    source: String,
    resolution: String,
    encoding: String,
    release_team: String,
    audio: String,
    audio_channel: String,
    subtitle: String,
    depth: String,
}

pub fn parse_filename(filename: &str) -> Result<MediaInfo, Box<dyn std::error::Error>> {
    let parser = Parser::new(filename)?;
    let mut clean_filename = filename.to_string();

    let (year_match, year) = match_parser!(parser, match_year);
    clean_filename = clean_filename.replace(&year_match, "");

    let (source_match, source) = match_parser!(parser, match_source);
    clean_filename = clean_filename.replace(&source_match, "");

    let (resolution_match, resolution) = match_parser!(parser, match_resolution);
    clean_filename = clean_filename.replace(&resolution_match, "");

    let (encoding_match, encoding) = match_parser!(parser, match_encoding);
    clean_filename = clean_filename.replace(&encoding_match, "");

    let (release_team_match, release_team) = match_parser!(parser, match_release_team);
    clean_filename = clean_filename.replace(&release_team_match, "");

    let (audio_match, audio) = match_parser!(parser, match_audio);
    clean_filename = clean_filename.replace(&audio_match, "");

    let (audio_channel_match, audio_channel) = match_parser!(parser, match_channel);
    clean_filename = clean_filename.replace(&audio_channel_match, "");

    let (subtitle_match, subtitle) = match_parser!(parser, match_subtitle);
    clean_filename = clean_filename.replace(&subtitle_match, "");

    let (depth_match, depth) = match_parser!(parser, match_depth);
    clean_filename = clean_filename.replace(&depth_match, "");

    Ok(MediaInfo {
        title: clean_filename,
        year,
        source,
        resolution,
        encoding,
        release_team,
        audio,
        audio_channel,
        subtitle,
        depth,
    })
}
