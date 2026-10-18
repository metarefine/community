# RTK v1.0.0 维护者审核记录

- 候选文件：recipes/context/rtk/recipe.json
- 审核候选提交：c21cddeb413e756c53d1aa61e48e8de93ba598df
- 候选文件 SHA-256：875b04a725b08d6257ee3e42ee2b127c02ef775c46d8437a5f46ce6e26a007c4
- 状态：accepted-for-next-application-build/source-included
- 审核类型：maintainer-self-review-ai-assisted
- 审核者：MetaRefine maintainer workflow (AI-assisted, on behalf of Donald Tong)

该文件是 Donald Tong 的维护者示例，采用 AI 辅助整理；它不是第三方社区贡献，也没有独立人类或外部贡献者审核。维护者核对了候选来源、声明的方法与现有实现，并确认 recipes/LICENSE 中的 MIT 授权仅覆盖本例配方文件，不覆盖讨论内容或第三方材料。

候选固定为 rtk 引擎的 standard 步骤，适用环境为随包本地引擎上下文流水线，压缩组件版本为 3.8.51。fallback_scheme=standard 仅表示用户可手动切回 Standard；配置未确认时不自动切换或重试。

## 验证

- 4 项定向 Rust 检查各通过 1/1，覆盖固定步骤、被修改或 pending 配方的回退、metadata JSON 形状和 Runtime 配置保护。
- 加入 pending 负例后，仅重跑受影响检查，结果 1/1 通过。
- 前端标准构建通过。
- 隔离预览通过：Standard 默认状态、RTK 配方信息与来源说明、选择配方，以及手动返回 Standard；无页面错误。
- 付费模型调用：0。

## 交付状态

固定快照已收录到应用源码，供下一应用构建使用。尚未打包或发布；社区仓库合并本身不会让配方进入客户运行路径。