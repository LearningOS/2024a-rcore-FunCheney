use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct BankersAlgorithm {
    available: Vec<usize>,       // 可用资源
    allocation: Vec<Vec<usize>>,  // 每个线程当前的资源分配
    need: Vec<Vec<usize>>,        // 每个线程还需要的资源
}

impl BankersAlgorithm {
    pub fn new() ->Self {
        Self{
            available: Vec::new(),
            allocation: Vec::new(),
            need: Vec::new(),
        }
    }


    fn is_safe(&self) -> bool {
        // 可用资源
        let mut work = self.available.clone();
        let mut finish = vec![false; self.allocation.len()];
        let mut safe_sequence = vec![];

        // 遍历所有进程，尝试找到一个安全序列
        for _ in 0..self.allocation.len() {
            let mut found = false;

            for i in 0..self.allocation.len() {
                if !finish[i] && self.need[i].iter().enumerate().all(|(j, &n)| n <= work[j]) {
                    // 假设可以分配资源
                    for j in 0..work.len() {
                        work[j] += self.allocation[i][j];
                    }
                    safe_sequence.push(i);
                    finish[i] = true;
                    found = true;
                    break;
                }
            }

            // 如果在当前循环中没有找到可以分配的进程，则系统不安全
            if !found {
                println!("No safe sequence found.");
                return false;
            }
        }

        println!("Safe sequence: {:?}", safe_sequence);
        true
    }

    pub fn request_resources(&mut self, tid: usize, request: Vec<usize>) -> bool {
        // 检查请求是否超出该进程的最大需求
        if request.iter().enumerate().any(|(i, &r)| r > self.need[tid][i]) {
            println!("Error: Process {} requested more resources than needed.", tid);
            return false;
        }

        // 检查资源是否足够满足请求
        if request.iter().enumerate().any(|(i, &r)| r > self.available[i]) {
            println!("Resources not available for Process {}.", tid);
            return false;
        }

        // 试探性分配
        for i in 0..self.available.len() {
            self.available[i] -= request[i];
            self.allocation[tid][i] += request[i];
            self.need[tid][i] -= request[i];
        }

        // 检查系统是否处于安全状态
        if self.is_safe() {
            true
        } else {
            // 回滚操作
            for i in 0..self.available.len() {
                self.available[i] += request[i];
                self.allocation[tid][i] -= request[i];
                self.need[tid][i] += request[i];
            }
            println!("System would be unsafe; rolling back.");
            false
        }
    }

    // 释放资源
    pub fn release_resources(&mut self, tid: usize, release: Vec<usize>) {
        for i in 0..self.available.len() {
            self.available[i] += release[i];
            self.allocation[tid][i] -= release[i];
            self.need[tid][i] += release[i];
        }
    }
}