/*
 * @Description: 文件相关
 * @License: MIT License
 * @Author: Xinyi Liu(CairBin)
 * @version: 1.0.0
 * @Date: 2024-10-15 11:34:53
 * @LastEditors: Xinyi Liu(CairBin)
 * @LastEditTime: 2024-10-15 12:24:42
 * @Copyright: Copyright (c) 2024 Xinyi Liu(CairBin)
 */

#[derive(Debug)]
pub enum FileType{
    Ascii,
    Ebcdic,
    Image,
    NoPrint,  // 非打印
    Telnet,
}