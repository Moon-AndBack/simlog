# Simlog

这是一个开箱即用的日志组件，仅需要路径参数和是否在控制台打印，并且是否在控制台打印是pub的，你可以随时修改它，默认按天分割文件。</br>

## 例子

```rust
use std::{time, thread};
use simlog::Log;
let log = Log::new("./log", true);
loop {
    thread::sleep(time::Duration::from_secs(2));
    log.debug(&x);
    log.info(&x);
    log.warn(&x);
    log.error(&x);
    log.fatal(&x);
}
```

## 更新日志

- v1.0.3</br>
    修改日志文件名称规则，现在以日为单位。修复了windows下无法运行的问题，修改了Log的整体结构，去除过多的全局变量运行更加轻量。程序内置一条数量为1的线程池用来分割日志文件，如果有更好的办法请联系我谢谢！
