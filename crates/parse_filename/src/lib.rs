mod parser;
use crate::parser::Parser;

macro_rules! match_parser {
    ($parser:expr, $method:ident, $filename:expr) => {
        $parser
            .$method($filename)
            .unwrap_or_else(|| ("".to_string(), "".to_string()))
    };
}

pub fn parse_filename(filename: &str) {
    let parser = Parser::new();

    let (year_match, year) = match_parser!(parser, match_year, filename);
    let (source_match, source) = match_parser!(parser, match_source, filename);
    let (resolution_match, resolution) = match_parser!(parser, match_resolution, filename);
    let (encoding_match, encoding) = match_parser!(parser, match_encoding, filename);
    let (release_team_match, release_team) = match_parser!(parser, match_release_team, filename);
    let (audio_match, audio) = match_parser!(parser, match_audio, filename);
    let (subtitle_match, subtitle) = match_parser!(parser, match_subtitle, filename);
    let (depth_match, depth) = match_parser!(parser, match_depth, filename);

    println!("year: {} {}", year_match, year);
    println!("source: {} {}", source_match, source);
    println!("resolution: {} {}", resolution_match, resolution);
    println!("encoding: {} {}", encoding_match, encoding);
    println!("release_team: {} {}", release_team_match, release_team);
    println!("audio: {} {}", audio_match, audio);
    println!("subtitle: {} {}", subtitle_match, subtitle);
    println!("depth: {} {}", depth_match, depth);
}
