# Simlog

<p>这是一个开箱即用的日志组件，默认按天分割文件。</p>

## 例子

```rust
use simlog::Log;
//".log": 日志文件路径, "debug": 日志记录级别, true: 控制台打印
let log = Log::new("./log", "debug", true);
let temp_str = "todo!";

log.debug(temp_str);
log.info(temp_str);
log.warn(temp_str);
log.error(temp_str);
log.fatal(temp_str);
```

## 更新日志

- v2.0.2</br>
    显示被调用函数名

- v2.0.0</br>
    添加日志记录级别，new函数需要新的参数所以并不能向下兼容！</br>
    目前支持的级别：debug, info, warn, error, fatal


- v1.0.3</br>
    <p>
    修改日志文件名称规则，现在以日为单位。修复了windows下无法运行的问题，修改了Log的整体结构，去除过多的全局变量运行更加轻量。程序内置一条数量为1的线程池用来分割日志文件</p>
