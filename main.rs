use std::env;

mod ffmpeg;
mod file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("\nUsage: bilivdtool [video_path] [audio_path]\nMore Detail: https://github.com/aURORA-JC/bilivdtool\n");
    }
    let videopath: &String;
    let audiopath: &String;
    videopath = &args[1];
    audiopath = &args[2];
    file::vload(videopath);
    file::aload(audiopath);
    let mut p: std::path::PathBuf = env::current_dir().unwrap();
    p.push("out.mp4");
    ffmpeg::do_merge_operations("Vout.m4s", "Aout.m4s", p);
}
