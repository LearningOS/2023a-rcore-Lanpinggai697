# ch5 lab3实验报告

## 编程作业

实现 `spawn` 系统调用，使用进程名创建新 TaskControlBlock，将父进程设置为当前进程，并加入子进程表和等待运行队列，最后返回新子进程的 pid。

设置 `BIG_STRIDE` 为 `0x4000`，在 `TaskControlBlock` 里添加 `stride` 和 `priority`， 并且初始化。实现设置优先级的 `sys_set_priority()` 函数，在 `fetch()` 里使用 stride 调度，遍历所有进程找出最优先的。

将 lab2 `task/mod.rs` 中的 `add_syscall_count()`, `sys_task_info()`, `mmap()` 与 `munmap` 迁移到 `task/processor.rs`，另外改在 `processor.rs`里开启进程计时，在其他模块中的处理同 lab2。


## 问答作业

#### 1.

不是。在 p2 执行一个时间片后，由于使用 8bit 无符号整形储存 stride 导致的溢出，p2.stride = p2.stride+pass = 250 + 10 = 260 = 10 mod 256 < 255 = p1.stride，因此还是 p2 执行。

#### 2.

使用数学归纳法。所有进程初始的 stride 均为0.若一个进程被调度，由于优先级不小于2，增加 pass 后 stride 必然不大于 BigStride / 2。而若已经满足 STRIDE_MAX – STRIDE_MIN <= BigStride / 2, 原来的 STRIDE_MIN 增加的 pass 同样不大于 BigStride / 2，因而更新的 STRIDE_MAX 也不大于原来的 STRIDE_MIN + BigStride / 2。而且更新的 STRIDE_MIN 必然大于原来的 STRIDE_MIN，故更新后 STRIDE_MAX – STRIDE_MIN <= BigStride / 2依然成立，证毕。

#### 3.

```rust
use core::cmp::Ordering;

struct Stride(u64);

const BIG_STRIDE:u8 = 255;
const PASSMAX:u8 = 127;

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let stride1 = self.0 as u8;
        let stride2 = other.0 as u8;
        if stride1 > stride2 {
            let diff = stride1 - stride2;
            if diff > PASSMAX {
                Some(Ordering::Less)
            }
            else {
                Some(Ordering::Greater)
            }
        } 
        else {
            let diff = stride2 - stride1;
            if diff > PASSMAX {
                Some(Ordering::Greater)
            }
            else {
                Some(Ordering::Less)
            }
        }

    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

```

## 荣誉准则



1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

        无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

        无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。