/*
 * @Description: 线程池相关
 * @License: MIT License
 * @Author: Xinyi Liu(CairBin)
 * @version: 1.0.0
 * @Date: 2024-10-16 22:10:59
 * @LastEditors: Xinyi Liu(CairBin)
 * @LastEditTime: 2024-10-16 22:44:33
 * @Copyright: Copyright (c) 2024 Xinyi Liu(CairBin)
 */

use std::{error::Error, sync::{mpsc, Arc, Mutex}, thread::{self, JoinHandle}};


// 工作线程 
pub struct Worker where{
    id: usize,
    handler: Option<JoinHandle<()>>
}

impl Worker{
    pub fn new(id:usize, receiver: Arc::<Mutex<mpsc::Receiver<Task>>>)->Self{
        let t = thread::spawn(move || {
            loop{
                let task = receiver.lock().unwrap().recv().unwrap();
                match task{
                    Task::NewTask(f) => f(),
                    Task::End => break,
                };
            }
        });
    
        Self{
            id:id,
            handler:Some(t)
        }
    }

}

pub enum Task{
    NewTask(Box<dyn FnOnce() + 'static + Send>),
    End
}

// 线程池
pub struct ThreadPool{
    max_size: usize,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Task>
}

impl ThreadPool{
    pub fn new(max_size: usize)->Result<Self, Box<dyn Error>>{
        if max_size <= 0{
            return Err("max_size must be larger than zero".into());
        }

        let (tx,rx) = mpsc::channel();
        let mut workers:Vec<Worker> = Vec::with_capacity(max_size);
        let receiver = Arc::new(Mutex::new(rx));
        for i in 0..max_size{
            workers.push(Worker::new(
                i, Arc::clone(&receiver)
            ));
        }

        Ok(Self {
            workers: workers,
            max_size: max_size,
            sender: tx
        })
    }

    fn execute<F>(&self, func:F) where F:FnOnce() + 'static + Send{
        let task = Task::NewTask(Box::new(func));
        self.sender.send(task).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        for _ in 0..self.max_size{
            self.sender.send(Task::End).unwrap();
        }

        for i in self.workers.iter_mut(){
            if let Some(t) = i.handler.take(){
                t.join().unwrap();
            }
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_worker(){
        let pool = ThreadPool::new(3).unwrap();
        pool.execute(|| println!("thread 1"));
        pool.execute(|| println!("thread 2"));
        pool.execute(|| println!("thread 3"));
        pool.execute(|| println!("thread 4"));
        pool.execute(|| println!("thread 5"));
    }
}
