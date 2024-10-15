/*
 * @Description: 文件相关
 * @License: MIT License
 * @Author: Xinyi Liu(CairBin)
 * @version: 1.0.0
 * @Date: 2024-10-15 11:34:53
 * @LastEditors: Xinyi Liu(CairBin)
 * @LastEditTime: 2024-10-15 20:20:31
 * @Copyright: Copyright (c) 2024 Xinyi Liu(CairBin)
 */

use std::{error::Error, fs};

use walkdir::WalkDir;

#[derive(Debug)]
pub enum FileType{
    Ascii,
    Ebcdic,
    Image,
    NoPrint,  // 非打印
    Telnet,
}

pub trait IFileManager{
    /// 添加文件
    fn set_file(&mut self, path:&str)->Result<(), Box<dyn Error>>;
    /// 获取文件
    fn get_file(&self, path:&str)->Option<&std::fs::File>;
    /// 获取文件可变引用
    fn get_file_mut(&mut self, path:&str)->Option<&mut std::fs::File>;
    /// 判断文件是否存在
    fn is_exist(&self, path:&str)->bool;
    /// 清理所有文件缓存
    fn clear(&mut self);
    /// 获取文件数
    fn len(&self)->usize;
    /// 移除文件对象
    fn remove_file(&mut self, path:&str);
    /// 移除对象并删除文件
    fn remove_file_and_delete(&mut self, path:&str)->Result<(), Box<dyn Error>> ;
    /// 删除目录
    fn delete_dir(&mut self, path:&str)->Result<(), Box<dyn Error>>;
    /// 创建目录
    fn create_dir(&self, path:&str)->Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub struct FileManager{
    max_size: usize,
    files: std::collections::HashMap<String, std::fs::File>,
}

impl FileManager{
    pub fn new(max_size:usize)->Self{
        Self { 
            max_size: max_size, 
            files:  std::collections::HashMap::new(),
        }
    }
}

impl IFileManager for FileManager{
    fn set_file(&mut self, path:&str)->Result<(), Box<dyn Error>> {
        if self.files.contains_key(path){
            return Ok(());
        }

        let path_file = std::path::Path::new(path);
        let file = match path_file.exists(){
            true=>{
                let mut fp = std::fs::OpenOptions::new();
                fp.read(true).write(true).open(path_file)?
            },
            false=>{
                let mut fp = std::fs::OpenOptions::new();
                fp.read(true).write(true).create(true).open(path_file)?
            }
        };

        if self.files.len() >= self.max_size{
            self.clear();
        }
        self.files.insert(path.to_string(), file);

        Ok(())
    }

    fn get_file(&self, path:&str)->Option<&std::fs::File> {
        self.files.get(path)
    }

    fn get_file_mut(&mut self, path:&str)->Option<&mut std::fs::File>{
        self.files.get_mut(path)
    }

    fn is_exist(&self, path:&str)->bool {
        self.files.contains_key(path)
    }

    fn clear(&mut self) {
        self.files.clear();
    }

    fn len(&self)->usize {
        self.files.len()
    }
    
    fn remove_file(&mut self, path:&str) {
        if self.files.contains_key(path){
            self.files.remove(path);
        }
    }
    
    fn remove_file_and_delete(&mut self, path:&str)->Result<(), Box<dyn Error>> {
        self.remove_file(path);
        fs::remove_file(path)?;

        Ok(())
    }
    
    fn delete_dir(&mut self, dir_path:&str)->Result<(), Box<dyn Error>> {
        for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
    
            if path.is_file() {
                // 删除文件
                self.remove_file_and_delete(path.to_str().unwrap())?;
            } else if path.is_dir() && path.to_str().unwrap() != dir_path {
                // 删除空目录
                fs::remove_dir(path)?;
                println!("Deleted directory: {}", path.display());
            }
        }
    
        // 最后删除顶级目录
        fs::remove_dir_all(dir_path)?;

        Ok(())
    }
    
    fn create_dir(&self, path:&str)->Result<(), Box<dyn Error>> {
        fs::create_dir_all(path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use super::*;

    #[test]
    fn test_set_file(){
        let mut manager = FileManager::new(1);

        manager.set_file("temp/test1.txt").unwrap();
        assert_eq!(manager.len(), 1);

        // 触发自动清除
        manager.set_file("temp/test2.txt").unwrap();
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_read_file(){
        let mut manager = FileManager::new(1);

        manager.set_file("temp/test1.txt").unwrap();
        let mut buf = String::new();
        manager.get_file("temp/test1.txt")
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        assert_eq!(buf, "hello world");
    }

    #[test]
    fn test_write_file(){

        let mut manager = FileManager::new(1);
        manager.set_file("temp/test2.txt").unwrap();
        manager.get_file_mut("temp/test2.txt")
            .unwrap()
            .write_all("HIHIHI".as_bytes())
            .unwrap();
    }

    #[test]
    fn test_create_and_delete(){
        let mut manager = FileManager::new(1);
        manager.create_dir("temp/test_dir").unwrap();

        manager.set_file("temp/test_dir/test3.txt").unwrap();
        assert_eq!(manager.len(), 1);

        manager.delete_dir("temp/test_dir").unwrap();
        assert_eq!(manager.len(), 0);
    }

}