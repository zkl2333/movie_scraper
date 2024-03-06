fn main() {
    let filenames = vec![
        "[太极张三丰].Tai-chi.Master.1993.USA.BluRay.1080p.x265.10bit.AC3.2Audios-CMCT",
        "纵横四海.Once.A.Thief.1991.BluRay.1080p.x265.10bit.2Audios.MNHD-FRDS",
        "网络迷踪.Searching.2018.BluRay.1080p.x265.10bit.MNHD-FRDS",
        "背靠背,脸对脸.1994.1080p.x265.国语.简繁中字￡CMCT",
        "自由的幻影.The.Phantom.of.Liberty.1974.Criterion.Collection.1080p.BluRay.x265.10bit.FLAC.MNHD-FRDS",
        "芙蓉镇.Hibiscus.Town.1987.1080p.BluRay.x265.AC3￡cXcY@FRDS",
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
        let media_info = parse_filename::parse_filename(filename);
        println!("{:?}", media_info.unwrap());
    }
}
