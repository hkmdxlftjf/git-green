# git-green

基于工作量证明（PoW）的自动提交工具，用于保持 GitHub 提交记录“绿色”。程序每小时生成一个挑战字符串，并将其与递增 `nonce` 以及「前一次的哈希值」拼接后进行 `SHA-256` 计算（链式约束），当哈希的十六进制前缀满足 `k` 个 `0` 即视为求解成功，随后追加到 `pow.log` 并进行 Git 提交（可选推送）。基于 chinese_holiday 模块进行是否执行的判断。

使用说明：

- 运行一次当前小时求解：`cargo run -- --once`
- 以守护模式每小时运行：`cargo run`
- 配置难度：设置环境变量 `POW_DIFFICULTY`（默认 `2`）
- 提交后推送：设置环境变量 `GIT_GREEN_PUSH=1`
- 日志文件：`pow.log` 每次成功求解追加一行 JSON 记录

判断工作日：

- 通过 `chinese_holiday` 模块判断当天是否为工作日，工作日才执行 PoW 求解与提交。

## chinese_holiday 使用手册

```rust
fn main() {
    use chinese_holiday::*;

    assert_eq!(
        chinese_holiday(Ymd::new(2004, 1, 1)),
        DayKind::NewYearsDayHoliday
    );
    assert!(chinese_holiday(Ymd::new(2004, 1, 1)).is_holiday());

    assert_eq!(
        chinese_holiday(Ymd::new(2004, 5, 8)),
        DayKind::InternationalWorkersDayWorkday
    );
    assert!(chinese_holiday(Ymd::new(2004, 5, 8)).is_workday());
}
```

测试：

- 运行测试用例：`cargo test`
