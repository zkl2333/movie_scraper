extern crate regex;

mod parser;
use parser::*;

fn main() {
    let filenames = vec![
        "[太极张三丰].Tai-chi.Master.1993.USA.BluRay.1080p.x265.10bit.AC3.2Audios-CMCT",
        "纵横四海.Once.A.Thief.1991.BluRay.1080p.x265.10bit.2Audios.MNHD-FRDS",
        "网络迷踪.Searching.2018.BluRay.1080p.x265.10bit.MNHD-FRDS",
        "背靠背,脸对脸.1994.1080p.x265.国语.简繁中字￡CMCT",
        "自由的幻影.The.Phantom.of.Liberty.1974.Criterion.Collection.1080p.BluRay.x265.10bit.FLAC.MNHD-FRDS",
        "芙蓉镇.Hibiscus.Town.1987.1080p.BluRay.x265.AC3￡cXcY\\@FRDS",
        "芬奇.2021.ATVP.1080p.中英特效字幕￡CMCT老男孩",
        "范海辛.Van.Helsing.2004.BluRay.1080p.x265.10bit.2Audio.MNHD-FRDS",
        "落水狗.Reservoir.Dogs.1992.BluRay.1080p.x265.10bit.MNHD-FRDS",
        "血钻.Blood.Diamond.2006.BluRay.1080p.x265.10bit.2Audio.MNHD-FRDS",
        "迁徙的鸟.Le.Peuple.Migrateur.2001.Bluray.1080p.x265.10bit.4Audios.MNHD-FRDS",
        "金蝉脱壳.2013.1080p.国英双语.中英特效字幕￡CMCT老男孩",
        "雨人.Rain.Man.1988.BluRay.1080p.x265.10bit.2Audio.MNHD-FRDS",
        "黑天鹅.Black.Swan.2010.BluRay.1080p.x265.10bit.2Audio.MNHD-FRDS",
    ];

    for filename in filenames {
        parse_filename(&filename);
    }
}

fn parse_filename(filename: &str) {
    let parser = Parser::new();

    let year = match &parser.match_yaer(&filename) {
        Some(v) => v.to_owned(),
        None => "".to_string(),
    };
    let (source_match, source) = match &parser.match_source(&filename) {
        Some((k, v)) => (k.to_owned(), v.to_owned()),
        None => ("".to_string(), "".to_string()),
    };
    let (resolution_match, resolution) = match &parser.match_resolution(&filename) {
        Some((k, v)) => (k.to_owned(), v.to_owned()),
        None => ("".to_string(), "".to_string()),
    };
    let (encoding_match, encoding) = match &parser.match_encoding(&filename) {
        Some((k, v)) => (k.to_owned(), v.to_owned()),
        None => ("".to_string(), "".to_string()),
    };
    let (release_team_match, release_team) = match &parser.match_release_team(&filename) {
        Some((k, v)) => (k.to_owned(), v.to_owned()),
        None => ("".to_string(), "".to_string()),
    };
    let audio = parser.match_audio(&filename);
    let (audio_match, audio) = match &audio {
        Some((k, v)) => (k.to_owned(), v.to_owned()),
        None => ("".to_string(), "".to_string()),
    };
    let subtitle = parser.match_subtitle(&filename);
    let depth = parser.match_depth(&filename);

    println!(
        "filename: {}\nyear: {}\nsource: {}\nresolution: {}\nencoding: {}\nrelease_team: {}\naudio: {}\nsubtitle: {:?}\ndepth: {:?}\n",
        filename, year, source, resolution, encoding, release_team, audio, subtitle, depth
    );
}
