use std::env;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let offset: u64 = args[1].parse().expect("偏移量非法！");
    let mut input = File::open(&args[2]).expect("无法打开输入文件！");
    let mut output = File::create(&args[3]).expect("无法创建输出文件！");

    input.seek(SeekFrom::Start(offset)).expect("无法设置偏移量！");

    let mut buffer = [0;4096];
    loop {
        let bytes_read = input.read(&mut buffer).expect("文件读取失败！");
        if bytes_read == 0 { break; }
        output.write_all(&buffer[..bytes_read]).expect("写入文件失败！");
    }
}
