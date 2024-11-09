use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug)]
/// 银行家
pub struct BankersAlgorithm {
    available: Vec<usize>,       // 信号量可用数量
    allocation: Vec<Vec<usize>>, // 每个信号量给每个线程的分配情况
    need: Vec<Vec<usize>>,       // 每个线程对信号量的请求情况
}

impl BankersAlgorithm {
    /// 初始化
    pub fn new() -> Self {
        Self {
            available: Vec::new(),
            allocation: Vec::new(),
            need: Vec::new(),
        }
    }

    /// 确保 available、allocation 和 need 的大小足够
    fn ensure_capacity(&mut self, tid: usize, sem_id: usize) {
        // 扩展 available 的大小
        if sem_id >= self.available.len() {
            self.available.resize(sem_id + 1, 0);
        }
        // 扩展 allocation 的大小
        if tid >= self.allocation.len() {
            self.allocation
                .resize(tid + 1, vec![0; self.available.len()]);
        } else if sem_id >= self.allocation[tid].len() {
            self.allocation[tid].resize(sem_id + 1, 0);
        }
        // 扩展 need 的大小
        if tid >= self.need.len() {
            self.need.resize(tid + 1, vec![0; self.available.len()]);
        } else if sem_id >= self.need[tid].len() {
            self.need[tid].resize(sem_id + 1, 0);
        }
    }

    fn is_safe(&self) -> bool {
        let mut work = self.available.clone();
        let mut finished = vec![false; self.allocation.len()];
        let mut progress_made = true;

        while progress_made {
            progress_made = false;

            for (tid, alloc_vec) in self.allocation.iter().enumerate() {
                if finished[tid] {
                    continue;
                }

                let can_allocate = self.need[tid]
                    .iter()
                    .enumerate()
                    .all(|(sem_id, &need)| need <= work[sem_id]);

                if can_allocate {
                    for (sem_id, &alloc) in alloc_vec.iter().enumerate() {
                        work[sem_id] += alloc;
                    }
                    finished[tid] = true;
                    progress_made = true;
                }
            }
        }

        finished.iter().all(|&done| done)
    }

    /// 申请资源
    pub fn request_resources(&mut self, tid: usize, sem_id: usize) -> bool {
        self.ensure_capacity(tid, sem_id);

        if self.available[sem_id] == 0 {
            return true;
        }

        self.available[sem_id] -= 1;
        self.allocation[tid][sem_id] += 1;
        //self.need[tid][sem_id] -= 1;

        if self.is_safe() {
            true
        } else {
            self.available[sem_id] += 1;
            self.allocation[tid][sem_id] -= 1;
            self.need[tid][sem_id] += 1;
            println!("System would be unsafe; rolling back.");
            false
        }
    }

    /// 释放资源
    pub fn release_resources(&mut self, tid: usize, sem_id: usize) {
        self.ensure_capacity(tid, sem_id);

        if self.allocation[tid][sem_id] > 0 {
            self.available[sem_id] += 1;
            self.allocation[tid][sem_id] -= 1;
            self.need[tid][sem_id] += 1;
        }
    }

    /// 添加可用资源
    pub fn add_available(&mut self, _pid: usize, count: usize) {
        //if sem_id >= self.available.len() {
        //    self.available.resize(sem_id + 1, 0);
        //}
        self.available.push(count);
    }
}
