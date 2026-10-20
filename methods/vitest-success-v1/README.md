# Vitest 全通过摘要候选 v1

状态：accepted-for-next-application-build/source-included。Donald Tong 的原创候选已由 AI 辅助的维护者流程代表其完成自审；这不是独立人类或外部贡献者审核。来源和许可核对、必要测试已完成；应用源码已纳入下一构建，打包和发布尚未完成。

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

## 审核与验证记录

- 来源与许可：核对为 Donald Tong 的原创方法、AI 辅助；本目录 MIT 仅覆盖该方法源码和原创合成测试。
- 方法验证：三项新增 Rust 定向测试最终均通过。首轮有一项仅因合成 fixture 锚点不匹配失败；修正测试锚点后只重跑该失败项并通过，生产 parser 未因此修改。
- 后续格式修正：默认输出的 `Duration` 后有两个对齐空格，解析器和正样本已同步这一格式；版本、全通过、计数与拒绝条件保持原范围。三项既有方法及恢复定向测试通过。
- 原文保护回归：现有 Runtime `reducer_archive_failure_returns_original_and_success_keeps_recovery_stable` 定向测试通过，验证归档失败保留原文、成功归档仍可恢复。
- 本轮付费调用为 0。验证覆盖固定的合成格式和原文保护边界，不证明真实 Vitest 环境覆盖、任务质量或账单节省；候选已纳入下一构建源码，但尚未打包或发布。

## 后续应用验证边界

本说明中的来源核对、自审和定向测试记录按原样保留。它们说明方法解析器及原文保护边界，不替代应用集成后的真实客户端验证。方法进入新的固定应用版本前，应由 `mr-evals` 通过实际 MetaRefine Harness 运行对应的 WorkBuddy Vitest 任务，再复核该次证据并固定应用快照。本次文档同步没有新增 Harness 运行、成绩或审核结论。

