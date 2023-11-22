# 阶段3 lab1 实验报告

首先分析程序

```c
#include "syscall.h"

extern int write(int fd, const void *buf, int len);
extern void exit(int code);

int main(int argc, char *argv[]) {
    char greeting[11] = "my name is ";
    char error[15] = "Incorrect argc\\n";
    
    if (argc != 1) {
        write(1, error, 15);
        return 1;
    }
    int len = 0;
    while(argv[0][len] != 0) {
        len++;
    }
    write(1, greeting, 11);
    write(1, argv[0], len);
    return 0;
}
```

为什么会输出 Incorrect argc，当然是因为传入的 argc 有问题。

这个时候想到了实验指导书中提到的

![Untitled](https://persecution-1301196908.cos.ap-chongqing.myqcloud.com/image_bedUntitled.png)

rcore 采用的排布是不符合 elf 文件规范的，对比一下 ch7 中描述的 rcore 采用的排布

![Untitled](https://persecution-1301196908.cos.ap-chongqing.myqcloud.com/image_bed/Untitled.png)

可以发现完全不一样，而观察测例库中 lib/main.c

```c
#include "syscall.h"

extern int main(int argc, char *argv[]);
extern void exit(int code);

int __start_main(long *p)
{
	int argc = p[0];
	char **argv = (void *)(p+1);

	exit(main(argc, argv));
	return 0;
}
```

可以看出 main.c 中获取 argc 与 argv 的方式遵循 elf 规范，而不是 rcore 的规范。其实理论上这里我们修改 main.c 也是可以过的，让其遵守 rcore 的一套规范即可。

这里注意到 rcore 这套规范 argc 是不保存在栈上的，观察代码发现其保存在了 x10 x11 两个寄存器

![202311231](https://persecution-1301196908.cos.ap-chongqing.myqcloud.com/image_bed/202311231.png)

只要仍然将 argc 和 argv 指针保存在这两个寄存器，就仍然可以兼容之前的 rust 用户态测例

(话说 cstring 和 rust 的 string 编码不太一样吧，直接从 rust 传字符串给 c 能行吗，现在应该暂时不用考虑这么多？）

![202311232](https://persecution-1301196908.cos.ap-chongqing.myqcloud.com/image_bed/202311232.png)

然后要注意对齐的两行要删掉，因为 user_sp 需要指向 argc。

## 题外话

之前一直用 macos 做 lab，但是这次因为没找到mac上的 `riscv64-linux-musl-cross` 交叉编译工具链，并且 macos 上没有用户态 qemu 无法进行对拍，所以用了 docker。