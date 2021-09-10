该项目用于描述

1. x86 以及 x64 下的所有系统寄存器

这些寄存器的读写大多是有条件的，属于非安全函数； 需要在此基础上建立系统状态控制管理工具，以便安全地更改系统硬件状态。

消除差异的方式：

1. 宽度不同，则利用 usize 在 x86 和 x64 下的不同宽度；
2. 同一个 bit 字段在 x86 和 x64 两种模式下的位置不同，则使用条件编译宏；

> 尽量减少类型参数；因为强类型安全在这一层级没有什么意义，这里仅仅提供操作；

x86 和 x64 不是硬件 CPU 运行模式的区分，而是对其基础寄存器宽度或可运行的指令的区分。

### 关于写寄存器

所有寄存器均采用读-修改-写入流程，而非直接对寄存器修改。
如果直接对寄存器操作，可能会使寄存器处于初始值和目标值的中间状态，而这些值可能会有副作用。
