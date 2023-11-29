//! Constants in the kernel

#[allow(unused)]

/// user app's stack size
pub const USER_STACK_SIZE: usize = 4096 * 8;
/// kernel stack size
pub const KERNEL_STACK_SIZE: usize = 4096 * 8;
/// kernel heap size
pub const KERNEL_HEAP_SIZE: usize = 0x200_0000;

/// page size : 4KB
pub const PAGE_SIZE: usize = 0x1000;
/// page size bits: 12
pub const PAGE_SIZE_BITS: usize = 0xc;
/// the max number of syscall
pub const MAX_SYSCALL_NUM: usize = 500;
/// the virtual addr of trapoline
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
/// the virtual addr of trap context
pub const TRAP_CONTEXT_BASE: usize = TRAMPOLINE - PAGE_SIZE;
/// clock frequency
pub const CLOCK_FREQ: usize = 12500000;
/// the physical memory end
pub const MEMORY_END: usize = 0x88000000;
/// The base address of control registers in Virtio_Block device
pub const MMIO: &[(usize, usize)] = &[(0x10001000, 0x1000)];

/// 初始用户栈大小，用于存放 argc/argv/envs/auxv
pub const USER_INIT_STACK_SIZE: usize = 0x4000; // 16 KB,
/// 用户栈底位置。同时也是最开始的用户堆顶位置
pub const USER_STACK_OFFSET: usize = 0x4000_0000 - USER_STACK_SIZE;
/// 用户地址最大不能超过这个值
pub const USER_VIRT_ADDR_LIMIT: usize = 0xFFFF_FFFF;
/// 如果 elf 的 phdr 指示 base 是 0(如 libc-test 的 libc.so)，则需要找一个非0的位置放置
pub const ELF_BASE_RELOCATE: usize = 0x400_0000;