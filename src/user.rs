/*
 * @Description: 用户相关
 * @License: MIT License
 * @Author: Xinyi Liu(CairBin)
 * @version: 
 * @Date: 2024-10-15 20:25:30
 * @LastEditors: Xinyi Liu(CairBin)
 * @LastEditTime: 2024-10-15 21:00:12
 * @Copyright: Copyright (c) 2024 Xinyi Liu(CairBin)
 */
// 此处参考了 https://github.com/powerfooI/rftp/blob/master/src/lib/user.rs

use std::{error::Error, net::SocketAddr};

use crate::{file, transfer::Transfer};


/// 用户状态
#[derive(Debug)]
pub enum UserState {
    Logging,
    Online,
    Offline
}

/// 用户
#[derive(Debug)]
pub struct User{
    pub username: String,   // 用户名，标识用户
    pub state: UserState,    // 用户状态
    pub address: SocketAddr,
    pub transfer: Option<Arc<Mutex<ITransfer>>>,
    pub transfer_type: file::FileType,
    pub home: String,
}

impl User{
    pub fn new(username:&str, address:SocketAddr, home:&str){
        Self { 
            username: username.to_string(), 
            state: UserState::Logging, 
            address, 
            transfer: None, 
            transfer_type: file::FileType::Unknown, 
            home: home.to_string(), 
        }
    }

    pub fn new_anonymous(address:SocketAddr, home:&str){
        Self { 
            username: "anonymous".to_string(), 
            state: UserState::Online, 
            address, 
            transfer: None, 
            transfer_type: file::FileType::Unknown, 
            home: home.to_string(), 
        }
    }

    pub fn set_transfer(&mut self, trans:Transfer){
        self.transfer = Some(Arc::new(Mutex::new(trans)));
    }

    pub fn get_transfer(&self)->Result<Arc<Mutex<Transfer>>, Box<dyn Error>>{
        match self.transfer.clone(){
            Some(t) => Ok(t),
            None => Err("Transfer not found".into())
        }
    }
}