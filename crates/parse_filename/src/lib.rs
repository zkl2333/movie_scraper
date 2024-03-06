mod parser;
use crate::parser::Parser;

pub fn parse_filename(filename: &str) {
    let parser = Parser::new();

    let year = match &parser.match_yaer(filename) {
        Some(v) => v.to_owned(),
        None => "".to_string(),
    };
    let (source_match, source) = match &parser.match_source(filename) {
        Some((k, v)) => (k.to_owned(), v.to_owned()),
        None => ("".to_string(), "".to_string()),
    };
    let (resolution_match, resolution) = match &parser.match_resolution(filename) {
        Some((k, v)) => (k.to_owned(), v.to_owned()),
        None => ("".to_string(), "".to_string()),
    };
    let (encoding_match, encoding) = match &parser.match_encoding(filename) {
        Some((k, v)) => (k.to_owned(), v.to_owned()),
        None => ("".to_string(), "".to_string()),
    };
    let (release_team_match, release_team) = match &parser.match_release_team(filename) {
        Some((k, v)) => (k.to_owned(), v.to_owned()),
        None => ("".to_string(), "".to_string()),
    };
    let audio = parser.match_audio(filename);
    let (audio_match, audio) = match &audio {
        Some((k, v)) => (k.to_owned(), v.to_owned()),
        None => ("".to_string(), "".to_string()),
    };
    let subtitle = parser.match_subtitle(filename);
    let depth = parser.match_depth(filename);

    println!(
        "filename: {}\nyear: {}\nsource: {}\nresolution: {}\nencoding: {}\nrelease_team: {}\naudio: {}\nsubtitle: {:?}\ndepth: {:?}\n",
        filename, year, source, resolution, encoding, release_team, audio, subtitle, depth
    );
}
