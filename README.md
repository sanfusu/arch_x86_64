该项目用于描述

1. x86 以及 x64 下的所有系统寄存器

这些寄存器的读写大多是有条件的，属于非安全函数； 需要在此基础上建立系统状态控制管理工具，以便安全地更改系统硬件状态。

消除差异的方式：

1. 宽度不同，则利用 usize 在 x86 和 x64 下的不同宽度；
2. 同一个 bit 字段在 x86 和 x64 两种模式下的位置不同，则使用条件编译宏；

> 尽量减少类型参数；因为强类型安全在这一层级没有什么意义，这里仅仅提供操作；
>
> 更安全的操作放在 arch_x64 仓库中，可见：<https://gitee.com/sanfusu/arch_x64>，目前仅仅是示例。

### 关于写寄存器

所有寄存器均采用读-修改-写入流程，而非直接对寄存器修改。
如果直接对寄存器操作，可能会使寄存器处于初始值和目标值的中间状态，而这些值可能会有副作用。

### 关于 x86 和 x64

在 64-bit mode 为激活之前，只能运行使用 x86 target 编译的指令，因为 rax 等寄存器在 64-bit mode 之前并不可见。

但需要注意的是：对于 x64 平台，uefi 启动后会直接处于 long mode 下的 64-bit 模式。

所谓的兼容模式：允许 64-bit 操作系统在 long mode 下运行 32-bit 的应用。

- arch_x64 编译 target 为 x86_64
- arch_x86 编译 target 为 ix86

代码段描述符的 L bit 是否为 1 最终决定了所运行的代码是否处于 64-bit 模式。

### 如何从实模式进入到 64-bit 模式

在进入 64-bit 模式之前所有的代码编译的目标为 ix86，即按 32-bit 模式编译。进入到 Long 模式后，会首先处于兼容模式，这时将 按 64-bit 模式编译的代码段载入内存后，置该代码段描述符 L bit 为 1，并跳转到该代码段执行代码，就此进入到 64-bit 模式。
