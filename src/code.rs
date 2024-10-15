/*
 * @Description: 相应码以及消息
 * @License: MIT License
 * @Author: Xinyi Liu(CairBin)
 * @version: 1.0.0
 * @Date: 2024-10-15 21:05:10
 * @LastEditors: Xinyi Liu(CairBin)
 * @LastEditTime: 2024-10-15 21:33:52
 * @Copyright: Copyright (c) 2024 Xinyi Liu(CairBin)
 */

pub fn fmt_msg(code:u32, message:&str)->String{
    format!("{} {}\r\n", code, message).to_string()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_fmt_msg(){
        assert_eq!(
            "220 FTP server ready.\r\n",
            fmt_msg(220, "FTP server ready.")
        );
    }
}