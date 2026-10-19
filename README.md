# MetaRefine Community

这里是 MetaRefine 的公开反馈与交流入口，不包含 MetaRefine 应用源码。

通用 Bug、功能建议与兼容请求请使用 [Issues 对应表单](https://github.com/metarefine/community/issues/new/choose)；使用问答与经验交流请前往 [Discussions](https://github.com/metarefine/community/discussions)。维护者会根据复现与验收推进问题，公开状态不等于交付日期承诺。

**提交公开 Issue/Discussion 后，内容会对互联网公开。** 请勿填写密码、API/本机令牌、账户或订单信息、客户聊天、客户代码、绝对路径、完整日志、Archive/数据库或未脱敏截图。表单不能阻止用户误贴敏感信息，请提交前检查。

账号、账务、私密日志与客户资料请使用应用内私密反馈或已有的客户支持渠道，见 [SUPPORT.md](SUPPORT.md)。安全问题见 [SECURITY.md](SECURITY.md)，不要公开可利用细节。

当前功能与支持级别以实际产品版本说明为准；提出适配请求不代表已经支持对应工具。修复代码合并、测试版可用、正式版已发布是不同状态。

贡献方式见 [CONTRIBUTING.md](CONTRIBUTING.md)。

## 声明式配方提案

配方通过 fork 和 Pull Request 作为声明式 JSON 提交。候选文件不会自动进入客户执行路径；维护者核对来源、许可和与现有方法的关系，并完成最小验证后，才会显式收录到应用。应用收录、打包和正式发布分别记录。

当前 RTK 文件是 Donald Tong 的维护者示例，由 AI 辅助；它不是第三方社区贡献，也不是独立人审。维护者审核和必要验证已完成，固定快照已收录到应用源码，状态为 accepted-for-next-application-build/source-included；尚未打包或发布。配方中的 fallback_scheme=standard 表示用户手动切回 Standard；配置未确认时不会自动切换或重试。详见[审核记录](recipes/context/rtk/review.md)。公开配方不会因为合并到本仓库就自动在客户电脑上运行。

原创方法源码和公开合成测试也可通过同一 fork+PR 入口提交；外部提交不会自动执行。当前 [Vitest 成功输出摘要候选](methods/vitest-success-v1/README.md) 已完成来源、许可和必要验证核对，由 AI 辅助的维护者流程代表 Donald Tong 自审，不是独立人类审核。状态为 accepted-for-next-application-build/source-included；打包和发布尚未完成。
