# Vitest 全通过摘要候选 v1

状态：待维护者验证。当前候选是 Donald Tong 的原创方法说明，由 AI 辅助整理；不代表已独立审核、纳入应用或发布。

## 方法范围

候选接口为一个无副作用的 Rust 纯函数：

```rust
fn summarize_vitest_success(
    command: &str,
    text: &str,
    exit_status: i32,
) -> Option<String>
```

它只尝试把明确的 Vitest 全通过输出归纳成短摘要。任何条件不完全匹配时返回 `None`，由原有路径继续处理；不执行命令、不读取配置，也不推断未知结果。

## 拟议的严格接受条件

- command 必须精确为 `vitest run` 或 `npx vitest run`，且 exit_status 为 0。
- text 必须是未着色的纯文本，符合 Vitest 4.0.0 默认的多文件全通过输出；至少有两个文件通过行，各行计数必须与最终汇总一致。
- 标题、通过明细、汇总、`Start at` 和 `Duration` 均须符合固定格式；时长只接受 Vitest 默认的紧凑毫秒格式。
- 其他版本、ANSI 颜色、未知行、警告、skip、失败、计数不一致、非零退出状态或单独的 CR 换行均返回 `None`。

接入时继续使用现有加密 Archive/Recovery 保留原始文本。关闭恢复或归档失败时，必须保留原文并交回旧路径，不能用摘要替换未受保护的原文。

## 来源、许可与验证状态

方法由 Donald Tong 原创，AI 辅助整理；MIT 范围见本目录的 `LICENSE`，仅覆盖本方法源码和本候选的原创合成测试。

当前只准备合成输出格式边界。候选仍待维护者验证；这不证明实际 Vitest 环境覆盖、任务质量或任何账单节省，也不表示代码已集成到应用。
