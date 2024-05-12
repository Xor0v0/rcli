# RCLI

RCLI is a rust tool.

## 知识点

### Clap

1. `clap` crate 可以帮助构建 CLI 工具, 有两种构建方式, 一种是使用冗长的代码构建, 另一种是利用 `derive` feature. 本例子中采用后者: `cargo add clap --features derive`;

2. 在 clap 中, 一个 CLI 工具就是一个实现了 `Parser` trait 的 struct, 这个 struct 就是主命令 (command), 主命令下可以有多个子命令 (subcommand). 开发者可以为所有的命令标注 `#[command(...)]` attribute 为主命令自动生成一些信息, 比如 `#[command(version, name, author)]`. 「这里只能打印出来 version」

    [!TODO] 后续学习一下如何打印出命令的其他信息

    ```rust
    #[derive(Debug, Parser)]
    #[command(name = "rcli", version, author)]
    ```

3. clap 所有命令都可以附加参数, 标注 `#[arg()]` attribute 可以控制命令参数的行为, 比如生成参数的简写 (short), 指定默认值 (default_value 或者 default_value_t)等行为;

    ```rust
    struct CsvOpt {
        #[arg(short, long, value_parser = verify_input_file)]
        pub input: String,
        #[arg(short, long, default_value = "output.json")]
        pub output: String,
    }
    ```

4. clap 支持子命令, 用包含多个子命令状态的 enum 表示, 并且需要标注 `#[command(subcommand)]` attribute. 在 enum 内部指定各种子命令状态, 同样需要标注 `#[command(name, ...)]` 指定命令信息, 每个子命令状态也是用一个 struct 表示, 也可以标注 `#[arg()]` attribute 来生成参数简写等功能.

5. 命令参数中如果标注了 default_value 则表示可以对默认值执行 `into()` 操作, 使其符合要求; 而标注为 default_value_t 则直接使用字面量类型, 不做任何类型转换.

6. 对命令参数标注 `#[arg(value_parser = func_name)]` 可以使用系统自带或者自定义的参数检查函数 (cleanup) . 「不要相信任何用户输入」

### csv

1.  为了解析 csv 文件, 需要导入 `cargo add csv`.
2.  使用 `use csv::Reader` 用于获得文件 reader instance, 这个实例可以使用反序列化恢复出目标数据结构内容, 得到的是一个 Result 数组.

    ```rust
        let mut reader = Reader::from_path(input);
        let mut ret = Vec::with_capacity(128);
        for result in reader.deserialize() {
            let record: Player = res?;
            ret.push(record);
        }
    ```
3.  csv Reader 对象提供了很多序列化字符串获取方法, 我们需要了解 csv 文件:

    | Headers |
    |-|
    | Records |

    csv 文件由 Headers 和 Records 两部分组成, Records 就是每一条记录, Headers 记录了每一条记录中对应的字段. 可以类比于 Excel 表格.

    于是我们不仅可以通过 `deserialize` 方法直接获取反序列化之后 records 数据结构信息 ( records 字段直接跟数据结构中的字段对应), 还可以分别拿 headers 和 records 的反序列化出来的对象迭代器, 然后 zip 映射成 pair 元组的迭代器, 最后使用 collect() 把迭代器转换成 serde-json crate 定义的 Value 数组.

    - reader.headers(): 返回解析器读到的第一行内容「即表头字段」的引用 `Result<&StringRecord>`;
    - reader.records(): 返回所有 records 字符串的迭代器, 迭代器中每个元素都是一个 `Result<StringRecord, Error>`

    于是我们可以使用这个 json Value 数组序列化到一个 json 文件中.
    ```rust
        let mut reader = Reader::from_path(input);
        let mut ret = Vec::with_capciity(128);
        let mut headers = reader.headers()?.clone();
        for result in reader.records() {
            record = result?;
            let json_value = headers.iter().zip(record.iter()).collect::<Value>();
            ret.push(json_value);
        }
    ```

    显然, 后者的实现思路更具一般性.

### serde-json

1. `serde-json` crate 提供了把数据结构序列化保存到 json 文件中的功能.

2. 在这个例子中使用到了 `Value` 类型, 它表示了一个合法的 JSON value. **注意, 这里的 Value 实现了 Serialize trait, 因此可以序列化成不同的字符串格式, 而不仅限于 json 字符串**

3. `serde_json::to_string_pretty()` 函数可以把 JSON Value 数组序列化成字符串.

    ```rust
        let json = serde_json::to_string_pretty();
        fs::write(output, json);
    ```

### Serde

1.  我们需要 serde crate 提供的宏来标注可序列化的数据结构, 因此需要: `cargo add serde --features derive`

2.  `#[serde(rename="xx")]` attribute 用于建立数据结构字段到序列化字符串之间的映射关系.

    ```rust
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename = "PascalCase")]
    struct Player {}
    ```

    但这样定义映射就使得代码不具有一般性, 考虑使用 csv crate 中分别反序列化的方式


### Anyhow

1. 使用 anyhow 做错误处理的好处是: `Anyhow::Error`实现了大部分 `std::Result` 类型的 `Err<e>`(e 绝大部分情况都是不同的错误类型) 转换. 而 `?` 操作符实质上相当于一个 match pattern, 它会执行 `Err(e) => return Error(e.into())` , 因此当程序中需要 return 各种错误, 我们可以使用 `?` 来捕获错误类型并转换成 `anyhow::Error`类型. `anyhow::Error` 类型实现了 `display` trait, 因此可以被作为错误流打印.


### rand

1. rand crate 提供 rng 生成伪随机数, 提供 `random()` 函数为任意**基础类型**生成随机值. 使用 rng 需要注意: 一个 rng 确定了之后要生成的所有数据, 即你使用 rng 可以生成一个规模为 10 的数组, 那么你把 rng 发送给另一个人, 他生成的规模为 10 的数组应该和你生成的是相等的.
2. 如果对随机数安全强度要求不高, 可以直接使用 `rand::thread_rng()`, 如果对安全要求较高, 则可以选择实现了 `rand::CryptoRng` trait 的其他 rng.
3. rand crate 支持对一个 Vector 类型进行 shuffle, 还支持对从 Vector 中随机挑选一个元素. 使用二者都需要引入 `rand::seq::SliceRandom` 即可.


### base64

1. base64 crate 提供使用 `Engine` 去进行编解码, 我们可以对 `Engine` 进行一些个性化配置, 比如使用的 alphabet 或者 padding 形式.
2. crate 内置了一些具有良好配置的 Engine, 比如 `STANDARD` 这个 engine， 它提供最标准的 alphabet : `alphabet::STANDARD` 和 PAD config；另一个常用 engine 是 `URL_SAFE_NO_PAD`，它使用 `alphabet::URL_SAFE` 字母表和 NO_PAD config。
3. 使用时直接 `use base64::prelude::*`

### Misc
1.  所有的基础类型 T 都实现了 `Option<T>` trait, 因此值有可能为 None 时, 就可以直接使用 Option(T) . 本例子中对 CLI 的输出定义为 `Option(String)`, 来处理未给定输出文件的情况.
2.  对于任意数据结构可以实现 FromStr trait, 然后就可以通过 `parse()` 方法完成从 &str 到目标数据类型的转换. 比如本项目中的 `OutputFormat`.
3.  如果一个文件中逻辑过于庞杂, 则需要考虑 Refactor ; 如果多条逻辑出现重复, 则考虑抽象成一个函数/方法. 注意重构的时候实际上就是把一个文件变成一个文件夹, 有两种写法: 一种是直接文件夹中建立 `mod.rs` 来声明各个子模块, 另一种是在文件夹同目录创建与文件夹同名的 rs 文件 (比如 `process.rs` ) . 后者是新推荐的写法.
