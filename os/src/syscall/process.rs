//! Process management syscalls
use core::mem::size_of;

use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_user_token, query_task_info, mmap, munmap,
    }, mm::translated_byte_buffer, timer::get_time_us,
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
    pub(crate) status: TaskStatus,
    /// The numbers of syscall called by task
    pub(crate) syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub(crate) time: usize,
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
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let _ts_translated = translated_byte_buffer(current_user_token(), _ts as *mut u8, size_of::<TimeVal>());
    let _ts_ptr = _ts_translated[0].as_ptr() as *mut TimeVal;
    let us = get_time_us();
    unsafe {
        (*_ts_ptr).sec = us / 1_000_000;
        (*_ts_ptr).usec = us % 1_000_000;
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let _ti_translated = translated_byte_buffer(current_user_token(), _ti as *mut u8, size_of::<TaskInfo>());
    let _ti_ptr = _ti_translated[0].as_ptr() as *mut TaskInfo;
    query_task_info(_ti_ptr);
    unsafe {
        (*_ti_ptr).time = get_time_us() / 1000;
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap");
    if mmap(start, len, port) { 0 } else { -1 }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    if munmap(start, len) { 0 } else { -1 }
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
