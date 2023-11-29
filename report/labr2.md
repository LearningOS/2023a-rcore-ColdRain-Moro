# 阶段3 lab2 实验报告

首先需要实现的第一个系统调用是 ioctl，查了 man pages，似乎是用来修改一些特殊文件的硬件参数，感觉用不上，直接返回 0。

![image-20231130000813705](https://persecution-1301196908.cos.ap-chongqing.myqcloud.com/image_bed/image-20231130000813705.png)

第二个需要实现的系统调用是 writev，也就是实验指导书中提到的唯一一个需要手动实现的系统调用，但也不用从头实现，简单调一下sys_write就可以了

![image-20231130001145164](https://persecution-1301196908.cos.ap-chongqing.myqcloud.com/image_bed/image-20231130001145164.png)

~~~rust
#[repr(C)]
pub struct Iovec {
    iov_base: *const u8,
    iov_len: usize
}

/// writev
/// iov 是一个 Iovec 结构体数组
pub fn sys_writev(fd: usize, iov: *const Iovec, iovcnt: usize) -> isize {
    trace!("kernel:pid[{}] sys_writev", current_task().unwrap().pid.0);
    let token = current_user_token();
    let mut ptr = iov;
    let mut cnt = 0;
    for _ in 0..iovcnt {
        let translated_ref = translated_ref(token, ptr);
        let res = sys_write(fd, translated_ref.iov_base, translated_ref.iov_len);
        if res == -1 {
            return -1;
        }
        cnt += res;
        unsafe {
            ptr = ptr.add(1);
        }
    }
    cnt
}
~~~

最后一个系统调用是 exit_group , man pages 里说是用于退出当前进程和当前进程的全部线程，只需要调 sys_exit 即可

## 踩坑

- 开启 debug 模式后内核栈大小不够用，进入某个函数后就没下文了，也没有提示panic的信息

## 问答题

![image-20231130002502016](https://persecution-1301196908.cos.ap-chongqing.myqcloud.com/image_bed/image-20231130002502016.png)

直接下载musl源码 全局搜索 man pages 中的宏名称即可。就是一堆 bit flags，没什么好说的。