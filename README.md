# RCLI

RCLI is a rust tool.

## 知识点

1. `clap` crate 可以帮助构建 CLI 工具, 通常使用 `derive` feature: `cargo add clap --features derive`;
2. 在 clap 中, 一个 CLI 工具就是一个实现了 `Parser` trait 的 struct, 在这个 struct 中可以定义各种命令 (包括子命令). 其中所有命令都可以附加参数, 使用 `#[arg()]` attribute 来标注参数选项, 以便生成命令参数的简写 (short), 默认值 (default_value 或者 default_value_t)等功能;
3. 当用户指定了某个命令, CLI 工具需要执行相应的动作; 一般而言, 命令参数都是基础类型, 并且开发者可以使用 `#[command(command_name)]` attribute 自动生成一些命令的动作, 比如 `#[command(version, name, author)]`.
4. clap 支持自定义子命令类型, 一个子命令类型就是一个包含各种状态的 enum, 并且需要标注 `#[command(subcommand)]` attribute. 在子命令内部可以同样的指定各种命令参数参数, 同样可以标注 `#[arg()]` attribute 来生成参数简写等功能.
5. 命令参数中如果标注了 default_value 则表示可以对默认值执行 `into()` 操作, 使其符合要求; 而标注为 default_value_t 则直接使用字面量类型, 不做任何类型转换.
6. 对命令参数标注 `#[arg(value_parser = func_name)]` 可以使用系统自带或者自定义的参数检查函数 (cleanup) . 「不要相信任何用户输入」
7. 使用 anyhow 做错误处理的好处是: 它实现了大部分 `std::Result` 类型的 Err 转换. 而 `?` 操作符实质上相当于一个 match pattern, 它会执行 `Err(e) => return Err(e.into())` , 因此当程序中需要 return 各种错误, 我们都可以使用 `?` 来捕获错误类型并转换成 `anyhow::Error`类型.
