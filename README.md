# soda_clix

## 介绍

识别和刮削媒体资源文件。

## 注意事项

- 不支持蓝光原盘。
- 刮削信息从github、tmdb、fanart获取，需要科学上网。
- 问题反馈
  - 从Github上提ISSUE并且附上日志文件。

## 如何安装

### 从Docker安装

```shell
docker pull biezhihua521/soda_clix
```

### 从cargo安装

```shell
cargo install soda_clix
```

### 从Github下载二进制文件

```text
https://github.com/biezhihua/soda-resource-tools/releases/tag/v0.1.1
```

## 如何使用


```shell
# 查看soda_clix版本

➜  ~ soda_clix --version
soda_clix 0.1.1
```

```shell
# 查看soda_clix帮助

➜  ~ soda_clix --help
A media scrape CLI

Usage: soda_clix [OPTIONS] <COMMAND>

Commands:
  scrape  刮削资源
  help    Print this message or the help of the given subcommand(s)

Options:
      --dev <DEV>                开发模式 [possible values: true, false]
      --log-path <LOG_PATH>      日志路径
      --log-level <LOG_LEVEL>    日志级别 [default: debug] [possible values: trace, debug, info, warn, error]
      --cache-path <CACHE_PATH>  缓存路径
  -h, --help                     Print help
  -V, --version                  Print version
```

```shell
# 查看soda_clix scrape子命令

➜  soda_clix scrape
刮削资源

Usage: soda_clix scrape [OPTIONS]

Options:
      --resource-type <RESOURCE_TYPE>  媒体类型 mt: 电影和电视剧 [default: mt] [possible values: mt]
      --transfer-type <TRANSFER_TYPE>  媒体从源目录转移到输出目录的方式 hard_link: 硬链接 symbol_link: 符号链接 copy: 复制 move: 移动 [default: hard_link] [possible values: hard_link, symbol_link, copy, move]
      --scrape-image                   刮削图片 true: 刮削图片 false: 不刮削图片
      --rename-style <RENAME_STYLE>    重命名格式 emby: Emby格式 [default: emby] [possible values: emby]
      --src <SRC>                      媒体源目录或文件
      --target <TARGET>                媒体刮削输出目录 刮削后的文件输出目录，如果不指定则默认为src
  -h, --help                           Print help                         Print help
```

```shell
# 刮削文件

➜ soda_clix scrape --resource-type mt --transfer-type hard_link --src ./xxx.mkv --target ./Target/电影
```

```shell
# 刮削目录

➜ soda_clix scrape --resource-type mt --transfer-type hard_link --src ./Src --target ./Target/电影
```


## 问题排查

### 网络问题

```text
➜  downloads soda_clix scrape --resource-type mt --transfer-type hard_link --src ./Spider.Man.Across.the.Spider.Verse.2023.2160p.WEB-DL.H.265.DDP.5.1.Atmos.mkv --target ./电影
2024-02-10 21:26:23  INFO soda::info: 配置文件目录: /root/.config/soda
2024-02-10 21:26:23  INFO soda::info: 缓存文件目录: /root/.cache/soda/cache
2024-02-10 21:26:23  INFO soda::info: 日志文件目录: /root/.cache/soda/log
2024-02-10 21:26:23  INFO soda::info: 开始检查网络
2024-02-10 21:26:23  INFO soda::info: 开始访问: https://raw.githubusercontent.com/biezhihua/soda-resource-tools/main/soda_cli_config/soda_config.json
2024-02-10 21:26:23  INFO soda::info: 开始访问: https://api.themoviedb.org
Error: Request(reqwest::Error { kind: Request, url: Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Domain("api.themoviedb.org")), port: None, path: "/", query: None, fragment: None }, source: TimedOut })
```

### 日志目录

#### Windows

```text
配置文件目录: C:\Users\biezhihua\AppData\Roaming\biezhihua\soda\config
缓存文件目录: C:\Users\biezhihua\AppData\Local\biezhihua\soda\cache\cache
日志文件目录: C:\Users\biezhihua\AppData\Local\biezhihua\soda\cache\log
```

#### linux 

```text
配置文件目录: /root/.config/soda
缓存文件目录: /root/.cache/soda/cache
日志文件目录: /root/.cache/soda/log
```

## 开发者

### 如何编译

```shell
cargo build
```

## 未来计划

- scrape的config配置拆分，每次刮削的config都应该是隔离的。

## 开源协议

```text
GPL-3.0 license
```