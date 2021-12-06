# Simlog

<p>这是一个开箱即用的日志组件，默认按天分割文件。</p>

## 例子

```rust
use simlog::{Log, Level};

#[test]
fn log() {
    // 路径留空关闭文件输出 Log::new("", Level::Debug, true);
    // 使用None级别关闭日志 Log::new("log", Level::None, true);
    // 使用false关闭控制台打印 Log::new("log", Level::Debug, false);
    let log = Log::new("log", Level::Debug, true);
    let temp_str = "todo!";

    log.debug(format!("{}", temp_str));
    log.info(format!("{}", temp_str));
    log.warn(format!("{}", temp_str));
    log.error(format!("{}", temp_str));
    log.fatal(format!("{}", temp_str));
}
```

## 更新日志

- v3.0.0</br>
    一些优化

- v2.0.0</br>
    添加日志记录级别</br>
    目前支持的级别：debug, info, warn, error, fatal


- v1.0.3</br>
    <p>
    修改日志文件名称规则，现在以日为单位。修复了windows下无法运行的问题，修改了Log的整体结构</p>
