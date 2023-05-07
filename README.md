1. let 是在栈上申请了一个变量
    如果是 `let t = Box::new();` 这种形式, 仍然有一个在栈上的指针指向了堆上  
2. 全局变量是储存在程序的**数据段**, 执行期间有一个不会改变的固定地址, 并且不需要堆栈空间, rust 希望非常明确的内存管理, 因此要使用 static(const 定义了常量, 只有static可以给予一个全局变量)
