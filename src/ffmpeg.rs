use core::panic;
use std::{fs, path::{Path, PathBuf}, process::Command};

pub fn do_merge_operations(videopath: &str, audiopath: &str, outputpath: PathBuf) {
    let mut cmd_str: String = String::new();
    let mut exepath: String = String::from("");
    let path = Path::new("./");
    let entries = fs::read_dir(path).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if path.to_str().unwrap() == "./ffmpeg.exe" {
                exepath = "ffmpeg.exe".to_string();
            }
        }
    }
    print!("检测系统中\n");
    if cfg!(target_os = "windows") {
        if exepath == "" {
            print!("检测完毕 为windows,且根目录没有ffmpeg");
            cmd_str.push_str("ffmpeg -y -i ");
            cmd_str.push_str(&videopath);
            cmd_str.push_str(" -i ");
            cmd_str.push_str(&audiopath);
            cmd_str.push_str(" -c:v copy -c:a copy ");
            cmd_str.push_str(&outputpath.display().to_string());

        } else {
            print!("检测完毕 为windows,且根目录有ffmpeg");
            cmd_str.push_str(&exepath);
            cmd_str.push_str(" -y -i ");
            cmd_str.push_str(&videopath);
            cmd_str.push_str(" -i ");
            cmd_str.push_str(&audiopath);
            cmd_str.push_str(" -c:v copy -c:a copy ");
            cmd_str.push_str(&outputpath.display().to_string());

        }
    } else {
        panic!("此系统不是Windows");
    }
    let output = Command::new("cmd")
                            .arg("/c")
                            .arg(cmd_str)
                            .output()
                            .expect("无法找到Cmd！");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    print!("完成转换\n");
    fs::remove_file("./Aout.m4s").unwrap();
    print!("删除音频缓存\n");
    fs::remove_file("./Vout.m4s").unwrap();
    print!("删除视频缓存");
}