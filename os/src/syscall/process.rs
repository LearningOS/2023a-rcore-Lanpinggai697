//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,
        mmap,munmap,current_user_token, get_current_task_info,
    },
    timer::get_time_us, mm::{translate_ptr, VirtAddr, VirtPageNum},
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
    //trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
//    trace!("kernel: sys_get_time");
    let time_kernel_ptr=translate_ptr(current_user_token(), _ts);
    let us=get_time_us();
    unsafe{
        *time_kernel_ptr=TimeVal{
            sec:us/1000000,
            usec:us%1000000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let task_info_kernel_ptr=translate_ptr(current_user_token(),_ti);
    let (status,syscall_times,time)=get_current_task_info();
    unsafe{
        *task_info_kernel_ptr=TaskInfo{
            status,
            syscall_times,
            time,
        };
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap");
    let start_va:VirtAddr=_start.into();
    if (!start_va.aligned())||(_port&7==0)||(_port>7){
        return -1;
    }

    if _len==0{
        return 0;
    }
    let end_va:VirtAddr=(_start+_len).into();
    let start_vpn:VirtPageNum=start_va.into();
    let end_vpn:VirtPageNum=end_va.ceil();

    mmap(start_vpn, end_vpn,_port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap");
    let start_va:VirtAddr=_start.into();
    if !start_va.aligned(){
        return -1;
    }

    if _len==0{
        return 0;
    }
    let end_va:VirtAddr=(_start+_len).into();
    let start_vpn:VirtPageNum=start_va.into();
    let end_vpn:VirtPageNum=end_va.ceil();

    munmap(start_vpn, end_vpn)
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
