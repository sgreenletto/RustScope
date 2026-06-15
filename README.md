# Rust 多线程日志统计器

## 一、项目简介

本项目是一个使用 Rust 实现的多线程日志统计器，主要用于练习 `Arc<T>` 和 `Mutex<T>` 在多线程场景下的使用。

程序会将日志数据分块交给多个子线程并发处理，并统计以下内容：

1. 每种日志级别出现的次数，例如 `INFO`、`WARN`、`ERROR`；
2. 每个服务出现 `ERROR` 日志的次数；
3. 每个服务下各日志级别出现的次数；
4. 格式错误而被跳过的日志数量。

本项目没有使用 `Rc<RefCell<T>>`，而是使用 `Arc<Mutex<HashMap<...>>>` 在线程之间共享并安全修改统计结果。

## 二、项目结构

```text
third/
├── Cargo.toml
├── Cargo.lock
├── README.md
└── src/
    └── main.rs
```

主要文件说明：

* `Cargo.toml`：Rust 项目配置文件；
* `Cargo.lock`：依赖版本锁定文件；
* `src/main.rs`：程序主要源代码；
* `README.md`：项目说明和运行方式。

## 三、运行环境

需要提前安装 Rust 环境。

可以在命令行中输入下面的命令检查是否安装成功：

```bash
rustc --version
cargo --version
```

如果能够正常显示版本号，说明 Rust 环境已经配置完成。

## 四、运行方式

进入项目根目录，例如：

```bash
cd third
```

然后运行：

```bash
cargo run
```

如果需要先格式化代码，可以运行：

```bash
cargo fmt
```

再运行：

```bash
cargo run
```

### 1. 分析当前项目

在项目根目录下运行：

```bash
cargo run -- analyze .
```

也可以使用简写形式：

```bash
cargo run -- .
```

其中 `.` 表示分析当前目录。

### 2. 分析指定 Rust 项目

也可以传入其他 Rust 项目的路径：

```bash
cargo run -- analyze "D:\study\rust程序设计\third"
```

如果路径中包含中文或空格，建议使用双引号包起来。

### 3. 生成 Markdown 报告

```bash
cargo run -- analyze . --format markdown --output report.md
```

运行后会在当前目录生成 `report.md`。

### 4. 生成 HTML 报告

```bash
cargo run -- analyze . --format html --output report.html
```

### 5. 启动 TUI 界面

```bash
cargo run -- tui .
```

## 示例运行结果

在项目根目录下运行：

```bash
cargo run -- analyze .
```

程序会输出当前项目的分析结果，例如项目文件数量、代码行数、结构信息等。

如果需要保存报告，可以运行：

```bash
cargo run -- analyze . --format markdown --output report.md
```

生成的 `report.md` 可以作为项目分析报告查看。

## 五、实现思路

程序使用 `thread::spawn` 创建多个子线程，每个线程负责处理一部分日志数据。

为了让多个线程共享统计结果，程序使用了 `Arc`。因为多个线程会同时修改 `HashMap`，所以又使用 `Mutex` 对共享数据进行保护，避免数据竞争。

程序中主要使用了以下共享统计结构：

```rust
Arc<Mutex<HashMap<String, usize>>>
```

用于保存：

* `level_count`：每种日志级别出现次数；
* `service_error_count`：每个服务出现 `ERROR` 日志的次数。

同时，程序还使用：

```rust
Arc<Mutex<HashMap<String, HashMap<String, usize>>>>
```

用于保存：

* `service_level_count`：每个服务下各日志级别出现的次数。

每个线程不是每处理一条日志就加锁，而是先使用本地 `HashMap` 统计自己负责的日志。线程处理完成后，再统一加锁，将本地统计结果合并到全局统计结果中。这样可以减少锁竞争。

主线程会使用 `join()` 等待所有子线程执行结束，然后再打印最终统计结果。

## 六、扩展功能

在基础要求之外，本项目还实现了以下扩展功能：

1. 统计每个服务下各日志级别出现次数，即 `service_level_count`；
2. 处理缺少 `service` 或 `level` 字段的格式错误日志，并统计 `skipped_count`；
3. 每个线程先进行本地统计，最后统一合并到共享结果中，减少锁竞争。

为了演示格式错误日志处理功能，程序在原始日志数据基础上额外加入了 2 条格式错误日志。

## 七、示例运行结果

运行：

```bash
cargo run
```

示例输出如下：

```text
level_count:
{
    "ERROR": 3,
    "INFO": 1,
    "WARN": 2,
}

service_error_count:
{
    "auth": 2,
    "payment": 1,
}

service_level_count:
{
    "auth": {
        "ERROR": 2,
    },
    "order": {
        "INFO": 1,
        "WARN": 1,
    },
    "payment": {
        "ERROR": 1,
        "WARN": 1,
    },
}

skipped_count:
2
```

说明：`HashMap` 本身不保证输出顺序，因此实际运行时字段顺序可能不同，但统计结果应保持一致。
