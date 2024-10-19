//! Process management syscalls

use crate::{
    config::MAX_SYSCALL_NUM,
    mm::{translated_physical_address, VirtAddr, VirtPageNum},
    task::{
        change_program_brk, exit_current_and_run_next, get_current_start_time, get_current_status,
        get_syscalls_time, mmap, suspend_current_and_run_next, unmap, TaskStatus,
    },
    timer::{get_time_ms, get_time_us},
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    // 系统调用发生在用户态下的指针
    let us = get_time_us();
    // 转换到物理地址
    let time_ptr = translated_physical_address(ts as *const u8) as *mut TimeVal;
    unsafe {
        *time_ptr = TimeVal {
            sec: us / 1000000,
            usec: us % 1000000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let task_status = get_current_status();
    let info_time = get_current_start_time();
    let syscall_times = get_syscalls_time();
    let time = get_time_ms();
    let ti_ptr = translated_physical_address(ti as *const u8) as *mut TaskInfo;
    unsafe {
        *ti_ptr = TaskInfo {
            status: task_status,
            syscall_times,
            time: time - info_time,
        };
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let start_vaddr: VirtAddr = _start.into();
    if !start_vaddr.aligned() {
        return -1;
    }

    if _port & !0x7 != 0 || _port & 0x7 == 0 {
        return -1;
    }

    if _len == 0 {
        return 0;
    }

    let end_vaddr: VirtAddr = (_start + _len).into();

    let start_vpn: VirtPageNum = start_vaddr.into();
    let end_vpn: VirtPageNum = end_vaddr.ceil();
    mmap(start_vpn, end_vpn, _port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    let start_vaddr: VirtAddr = _start.into();
    if !start_vaddr.aligned() {
        return -1;
    }

    let end_vaddr: VirtAddr = (_start + _len).into();

    let start_vpn: VirtPageNum = start_vaddr.into();
    let end_vpn: VirtPageNum = end_vaddr.ceil();
    unmap(start_vpn, end_vpn)
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
