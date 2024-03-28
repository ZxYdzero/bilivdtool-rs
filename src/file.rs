use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};


const REMOVE_BYTES: i32 = 9;

pub fn aload(apath: &String) {

    let mut output = String::from("./");
    output.push_str("Aout.m4s");
    let buffer_size = 1024 * 4; // 定义分块的大小，例如每次处理4KB

    // 打开输入文件
    let mut input_file = File::open(apath).unwrap();

    // 创建一个新文件用于写入剩余的数据
    let mut output_file = File::create(output).unwrap();

    // 跳过前面要删除的部分
    input_file.seek(SeekFrom::Start(REMOVE_BYTES as u64)).unwrap();

    // 创建一个缓冲区来分块读取文件
    let mut buffer = vec![0; buffer_size];

    // 循环读取文件的剩余部分
    loop {
        let bytes_read = input_file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break; // 文件读取结束
        }
        output_file.write_all(&buffer[..bytes_read]).unwrap(); // 将数据块写入新文件
    }
}

pub fn vload(vpath: &String) {

    let mut output = String::from("./");
    output.push_str("Vout.m4s");
    let buffer_size = 1024 * 4; // 定义分块的大小，例如每次处理4KB

    // 打开输入文件
    let mut input_file = File::open(vpath).unwrap();

    // 创建一个新文件用于写入剩余的数据
    let mut output_file = File::create(output).unwrap();

    // 跳过前面要删除的部分
    input_file.seek(SeekFrom::Start(REMOVE_BYTES as u64)).unwrap();

    // 创建一个缓冲区来分块读取文件
    let mut buffer = vec![0; buffer_size];

    // 循环读取文件的剩余部分
    loop {
        let bytes_read = input_file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break; // 文件读取结束
        }
        output_file.write_all(&buffer[..bytes_read]).unwrap(); // 将数据块写入新文件
    }
}