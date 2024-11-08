use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct BankersAlgorithm {
    available: Vec<usize>,       // 可用资源
    max: Vec<Vec<usize>>,         // 每个进程的最大需求
    allocation: Vec<Vec<usize>>,  // 每个进程当前的资源分配
    need: Vec<Vec<usize>>,        // 每个进程还需要的资源
}

impl BankersAlgorithm {
    fn new() ->Self {
        Self{
            available: Vec::new(),
            max: Vec::new(),
            allocation: Vec::new(),
            need: Vec::new(),
        }
    }

    // 检查系统是否处于安全状态
    fn is_safe(&self) -> bool {
        let mut work = self.available.clone();
        let mut finish = vec![false; self.max.len()];
        let mut safe_sequence = vec![];

        for _ in 0..self.max.len() {
            let mut found = false;
            for i in 0..self.max.len() {
                if !finish[i] && self.need[i].iter().enumerate().all(|(j, &n)| n <= work[j]) {
                    // 假设当前分配，更新工作量
                    for j in 0..work.len() {
                        work[j] += self.allocation[i][j];
                    }
                    safe_sequence.push(i);
                    finish[i] = true;
                    found = true;
                    break;
                }
            }
            if !found {
                return false; // 无安全序列，系统不安全
            }
        }
        println!("Safe sequence: {:?}", safe_sequence);
        true
    }

    // 请求资源
    pub fn request_resources(&mut self, process: usize, request: Vec<usize>) -> bool {
        if request.iter().enumerate().any(|(i, &r)| r > self.need[process][i]) {
            println!("Error: Process {} requested more resources than needed.", process);
            return false;
        }
        if request.iter().enumerate().any(|(i, &r)| r > self.available[i]) {
            println!("Resources not available for Process {}.", process);
            return false;
        }

        // 试探性分配
        for i in 0..self.available.len() {
            self.available[i] -= request[i];
            self.allocation[process][i] += request[i];
            self.need[process][i] -= request[i];
        }

        if self.is_safe() {
            true
        } else {
            // 回滚操作
            for i in 0..self.available.len() {
                self.available[i] += request[i];
                self.allocation[process][i] -= request[i];
                self.need[process][i] += request[i];
            }
            println!("System would be unsafe; rolling back.");
            false
        }
    }

    // 释放资源
    pub fn release_resources(&mut self, process: usize, release: Vec<usize>) {
        for i in 0..self.available.len() {
            self.available[i] += release[i];
            self.allocation[process][i] -= release[i];
            self.need[process][i] += release[i];
        }
    }
}