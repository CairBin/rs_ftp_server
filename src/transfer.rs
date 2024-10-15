/*
 * @Description: 文件传输相关
 * @License: MIT License
 * @Author: Xinyi Liu(CairBin)
 * @version: 1.0.0
 * @Date: 2024-10-15 20:36:29
 * @LastEditors: Xinyi Liu(CairBin)
 * @LastEditTime: 2024-10-15 21:29:42
 * @Copyright: Copyright (c) 2024 Xinyi Liu(CairBin)
 */
// 此部分参考了 https://github.com/powerfooI/rftp/blob/master/src/lib/session.rs

use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;

/// 传输模式
#[derive(Debug)]
pub enum TransferMode{
    /// 主动模式
    Port(Arc<Mutex<TcpStream>>),
    // 被动模式
    Pasv(Arc<Mutex<TcpStream>>),
}

#[derive(Debug)]
pub struct Transfer{
    pub mode: TransferMode,
    pub total_size:u64,
    pub finished_size:u64,
    pub offset:u64,
    pub filename: String,
    pub is_finished: bool,
    pub aborted: bool,
}

pub trait ITransfer{
    fn get_socket(&self)->Arc<Mutex<TcpStream>>;
}

impl Transfer{
    pub fn new(mode: TransferMode)->Self{
        Self { mode: mode, total_size: 0, finished_size: 0, offset: 0, filename: String::new(), is_finished: false, aborted: false }
    }
}

impl ITransfer for Transfer{
    fn get_socket(&self)->Arc<Mutex<TcpStream>> {
        match &self.mode{
            TransferMode::Port(s) => s.clone(),
            TransferMode::Pasv(s) => s.clone(),
        }
    }
}