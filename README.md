# Bilivdtool-rs
[aURORA-JC的bilivdtool工具](https://github.com/aURORA-JC/bilivdtool)的Rust版

# 特点
- 解码从哔哩哔哩下载的视频
- 并合并其音频与视频

## 注意
- 此工具极其依赖[FFmpeg](https://ffmpeg.org)，假如没有此工具，请前往[官方网站](https://ffmpeg.org/download.html)下载
- 安装后请确保环境变量的Path有FFmpeg
- 或者将FFmpeg的二进制文件放置与此工具相同的文件夹内

## 用法
```shell
bilivdtool [video_path] [audio_path]
```

## 构建
```shell
git clone https://github.com/ZxYdzero/bilivdtool-rs.git
cd ./bilivdtool-rs
cargo build --release
```

# 详细介绍请前往
- [此处](https://github.com/aURORA-JC/bilivdtool)

# License
(C) 2024 ZxYdzero. MIT License
