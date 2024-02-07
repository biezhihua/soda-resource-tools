# CLI

```shell
cargo run --bin soda_resource_tools_cl -- scrape --resource-type mt --transfer-type hard_link --src-dir D:\Downloads\Src --target-dir D:\Downloads\Test  
```

```shell
cargo run --bin soda_cli -- --log-path D:\Projects\github\soda-resource-tools\soda_resource_tools_cli\log --cache-path D:\Projects\github\soda-resource-tools\soda_resource_tools_lib\cache scrape --resource-type mt --transfer-type symbol_link --src-dir \\NAS-TANK\downloads_disk5\电视剧\Agatha.Christies.Poirot.S01-S13.COMPLETE.1080p.BluRay.REMUX.AVC.DTS-HD.MA_DD_FLAC.2.0_5.1-EPSiLON\Agatha.Christies.Poirot.S01.1080p.BluRay.REMUX.AVC.DD.2.0-EPSiLON --target-dir D:\Downloads\Target 
```

```shell
cargo run --bin soda_cli -- --log-path D:\Projects\github\soda-resource-tools\soda_resource_tools_cli\log --cache-path D:\Projects\github\soda-resource-tools\soda_resource_tools_lib\cache scrape --resource-type mt --transfer-type symbol_link --src-dir \\NAS-TANK\downloads_disk5\电视剧 --target-dir D:\Downloads\Target 
```

```shell
cargo run --bin soda_cli -- --log-path D:\Projects\github\soda-resource-tools\soda_resource_tools_cli\log --cache-path D:\Projects\github\soda-resource-tools\soda_resource_tools_lib\cache scrape --resource-type mt --transfer-type symbol_link --src-dir \\NAS-TANK\downloads_disk1\动漫 --target-dir D:\Downloads\Target 
```

```shell
cargo run --bin soda_cli -- --dev true scrape --resource-type mt --transfer-type symbol_link --src-dir \\NAS-TANK\downloads_disk5\电视剧 --target-dir D:\Downloads\Target 
```

```shell
cargo run --bin soda_cli -- --dev true scrape --resource-type mt --transfer-type symbol_link --src-dir \\NAS-TANK\downloads_disk1\动漫 --target-dir D:\Downloads\Target 
```

```shell
cargo run --bin soda_cli -- --dev true scrape --scrape-image false --resource-type mt --transfer-type symbol_link  --target-dir D:\Downloads\Target  --src-dir \\NAS-TANK\downloads_disk3\纪录片
```

```shell
cargo run --bin soda_cli -- --dev true scrape --scrape-image false --resource-type mt --transfer-type symbol_link --target-dir "D:\Downloads\Target" --src-dir "\\NAS-TANK\downloads_disk3/纪录片/宇宙 1-9 季[全集]The.Universe.S01-S09.2007-2015.BD-REMUX.1080p.H264.AVC.AC3-FrankB"
```

```shell
cargo run --bin soda_cli -- --dev true scrape --scrape-image false --resource-type mt --transfer-type symbol_link --target-dir "D:\Downloads\Target" --src-dir "\\NAS-TANK\downloads_disk3\纪录片\Forged.in.Fire.S08.720p.WEBRip.AAC2.0.x264-MIXED[rartv]"
```

```shell
cargo run --bin soda_cli -- --dev true scrape --scrape-image false --resource-type mt --transfer-type symbol_link --target-dir "D:\Downloads\Target" --src-dir "\\NAS-TANK\downloads_disk3\纪录片\Great.British.Railway.Journeys.S01-S13.720p.HDTV.x264-Mixed"
```

```shell
cargo run --bin soda_cli -- --dev true scrape --scrape-image false --resource-type mt --transfer-type symbol_link --target-dir "D:\Downloads\Target" --src-dir "\\NAS-TANK\downloads_disk3\纪录片\Air crash investigation（空中浩劫）全"
```

```shell
cargo run --bin soda_cli -- --dev true scrape --scrape-image false --resource-type mt --transfer-type symbol_link --target-dir "D:\Downloads\Target" --src-dir "\\NAS-TANK\downloads_disk3\纪录片\A.Perfect.Planet.S01.BluRay.2160p.Atmos.TrueHD.7.1.HDR.x265.10bit-CHD"
```

```shell
cargo run --bin soda_cli -- --dev true scrape --scrape-image false --resource-type mt --transfer-type symbol_link --target-dir "D:\Downloads\Target" --src-dir "\\NAS-TANK\downloads_disk3\纪录片\City.of.Angels.City.of.Death.S01.2021.Disney+.WEB-DL.1080p.H264.DDP-HDCTV"
```

```shell
cargo run --bin soda_cli -- --dev true scrape --scrape-image false --resource-type mt --transfer-type symbol_link --target-dir "D:\Downloads\Target" --src-dir "\\NAS-TANK\downloads_disk1\动漫"
```

```shell
cargo run --bin soda_cli -- --dev true scrape --scrape-image true --resource-type mt --transfer-type symbol_link --target-dir "D:\Downloads\Target" --src-dir "\\NAS-TANK\downloads_disk1\动漫"
```

```shell
cargo run --bin soda_cli -- --dev true scrape --scrape-image false --resource-type mt --transfer-type symbol_link --target-dir "D:\Downloads\Target" --src-dir "\\NAS-TANK\downloads_disk8\电影"
```

```shell
cargo run --bin soda_cli -- --dev true scrape --scrape-image true --resource-type mt --transfer-type symbol_link --target-dir "D:\Downloads\Target" --src-dir "\\NAS-TANK\downloads_disk3\纪录片"
```

```shell
cargo run --bin soda_cli -- --dev true scrape --scrape-image true --resource-type mt --rename-style emby --transfer-type symbol_link --target-dir "D:\Downloads\Target\电影" --src-dir "\\NAS-TANK\downloads_disk8\电影\Spider-Man.Across.the.Spider-Verse.2023.2160p.MA.WEB-DL.DDP5.1.Atmos.DV.HDR.H.265-FLUX.mkv"
```

```shell
 cargo run --bin soda_cli -- --dev true scrape --scrape-image true --resource-type mt --rename-style emby --transfer-type symbol_link --target-dir "D:\Downloads\Target\电影" --src-dir "\\NAS-TANK\downloads_disk8\电影\DouBan.2022.11.11.Top.250.BluRay.1080p.x265.10bit.MNHD-FRDS\东邪西毒终极版.Ashes.of.Time.Redux.2008.BluRay.1080p.x265.10bit.2Audio.MNHD-FRDS\Ashes.of.Time.Redux.2008.BluRay.1080p.x265.10bit.2Audio.MNHD-FRDS.mkv"
```

```
A resource manage CLI

Usage: soda.exe [OPTIONS] <COMMAND>

Commands:
  scrape  scrape resource
  help    Print this message or the help of the given subcommand(s)

Options:
      --cache-path <CACHE_PATH>
  -h, --help                     Print help
```

```
cargo run --bin soda -- --cache-path D:\Projects\github\soda-resource-tools\soda_resource_tools_lib\cache scrape --resource-type mt --transfer-type hard_link --src-dir D:\Downloads\Src

 cargo run --bin soda_cli -- --cache-path D:\Projects\github\soda-resource-tools\soda_resource_tools_lib\cache scrape --resource-type mt --transfer-type hard_link --src-dir D:\Downloads\Src
```