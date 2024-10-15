/*
 * @Description: FTP 命令解析
 * @License: MIT License
 * @Author: Xinyi Liu(CairBin)
 * @version: 1.0.0
 * @Date: 2024-10-15 11:00:22
 * @LastEditors: Xinyi Liu(CairBin)
 * @LastEditTime: 2024-10-15 12:34:37
 * @Copyright: Copyright (c) 2024 Xinyi Liu(CairBin)
 */

use std::{error::Error};
use std::net::{Ipv4Addr, SocketAddr};

use crate::file;


#[derive(Debug)]
pub enum FtpCommand{
    // 接入命令
    USER(String),   // 用户标识符
    PASS(String),   // 登陆口令
    QUIT,           // 系统注销
    ABOR,           // 数据连接以及上次命令终止

    // 文件管理命令
    CWD(String),    // 改变到另一个目录
    CDUP,           // 改变到父目录
    DELE(String),   // 删除文件
    LIST(Option<String>),   // 列出子目录和文件
    NLIST(Option<String>),  // 列出子目录或其它属性文件
    MKD(String),    // 创建目录
    PWD,            // 显示当前目录
    RMD(String),    // 删除目录
    RNFR(String),   // 标志要重新命名的文件，参数为旧文件名
    RNTO(String),   // 重命名，参数为新文件名
    TYPE(file::FileType),   // 定义文件类型

    // 端口相关
    PORT(SocketAddr),   // 客户端选择端口，主动模式
    PASV,               // 服务器选择端口，被动模式

    // 文件传输
    RETR(String),       // 下载文件
    STOR(String),       // 上传文件
    STOU(String),       // 上传文件，但是文件名必须唯一
    ALLO(u64),          // 在服务器为文件分配存储空间
    STAT(Option<String>),   // 返回文件状态
    REST(u64),          // 数据点给文件标记位置
    
    // 其他
    NOOP,             // 检查服务器是否工作
    SYST,             // 获取服务器所用OS
}


pub trait IFtpCommandParser{
    fn parse(msg: &str)->Result<FtpCommand, Box<dyn Error>>;
}


pub struct FtpCommandParser;

impl IFtpCommandParser for FtpCommandParser{
    fn parse(msg: &str)->Result<FtpCommand, Box<dyn Error>> {
        let mut iter = msg.trim().split_whitespace();
        let cmd = iter.next().unwrap();
        let args =  iter.collect::<Vec<&str>>().join(" ");


        Ok(match cmd{
            "USER" => FtpCommand::USER(args),
            "PASS" => FtpCommand::PASS(args),
            "QUIT" => FtpCommand::QUIT,
            "ABOR" => FtpCommand::ABOR,
            
            
            "CWD" => FtpCommand::CWD(args),
            "CDUP" => FtpCommand::CDUP,
            "DELE" => FtpCommand::DELE(args),
            "LIST" => FtpCommand::LIST(if args.is_empty() { None } else { Some(args) }),
            "NLIST" => FtpCommand::NLIST(if args.is_empty() { None } else { Some(args) }),
            "MKD" => FtpCommand::MKD(args),
            "PWD" => FtpCommand::PWD,
            "RMD" => FtpCommand::RMD(args),
            "RNFR" => FtpCommand::RNFR(args),
            "RNTO" => FtpCommand::RNTO(args),
            "TYPE" => {
                let ft: file::FileType = match args.as_ref() {
                    "A" => file::FileType::Ascii,
                    "E" => file::FileType::Ebcdic,
                    "I" => file::FileType::Image,
                    "N" => file::FileType::NoPrint,
                    "T" => file::FileType::Telnet,
                    _ => return Err("Invalid file type in FTP TYPE command".into()),
                };
                FtpCommand::TYPE(ft)
            },

            "PORT" => {
                // 拆分成六个部分（IP 地址四个部分，端口两个部分）
                let parts: Vec<u8> = args
                    .split(',')
                    .map(|s| s.parse::<u8>())
                    .collect::<Result<Vec<u8>, _>>()
                    .map_err(|_| "Invalid number in FTP PORT command".to_string())?;

                    if parts.len() != 6 {
                        return Err("Invalid FTP PORT command, expected 6 numbers".into());
                    }

                // 提取 IP 地址部分
                let ip = Ipv4Addr::new(parts[0], parts[1], parts[2], parts[3]);

                // 计算端口号：p1 * 256 + p2
                let port = (parts[4] as u16) * 256 + (parts[5] as u16);

                // 构造 SocketAddr
                FtpCommand::PORT(
                    SocketAddr::new(ip.into(), port)
                )
            },
            "PASV" => FtpCommand::PASV,

            "RETR" => FtpCommand::RETR(args),
            "STOR" => FtpCommand::STOR(args),
            "STOU" => FtpCommand::STOU(args),
            "ALLO" => FtpCommand::ALLO(args.parse::<u64>()?),
            "STAT" => FtpCommand::STAT(if args.is_empty() { None } else { Some(args) }),
            "REST" => FtpCommand::REST(args.parse::<u64>()?),

            "NOOP" => FtpCommand::NOOP,
            "SYST" => FtpCommand::SYST,

            _=>{
                FtpCommand::NOOP
            }
        })
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ftp_command() {
        let addr = match FtpCommandParser::parse("PORT 192,168,1,10,12,34").unwrap(){
            FtpCommand::PORT(s) => s,
            _ => panic!("Invalid command"),
        };

        assert_eq!(addr.to_string(), "192.168.1.10:3106");
    }
}