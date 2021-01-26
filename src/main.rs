use std::include_bytes;

// 都需要在入口处挂在导入包
mod psd;
mod sections;
use crate::psd::Psd;


fn main() {
    // 二进制 example.psd
    // let context = fs::read("example.psd").unwrap();
    // let context = fs::read_to_string("data.json").unwrap();
    let context = include_bytes!("../example.psd");
    // println!("context is : {:#?}", context);
    
    let psd = Psd::from_bytes(context);
    println!("fileHeader is : {:#?}", psd);
}
