当版主将评论标记为 reviewed 时触发。

### 代理接收的上下文

- 评论。
- 该 **triggering user ID** - 进行审核的版主。
- 可选的线程 / 用户历史 / 页面上下文（按配置）。

### 谁会触发此事件

版主在审核页面、评论小组件上或通过 API 执行的人工审核操作。

### 常见用途

- **Audit forwarding** 通过 [Webhooks](#webhooks-overview)。
- **Memory writes** - 记录一个记忆备注，说明此评论已由人工审核，以便其他代理不会重复处理。

### 注意事项

- "Reviewed" is one of the moderation queue states tracked separately from "approved" and "spam". A comment can be approved-and-reviewed, approved-but-not-reviewed, etc. See [审批的工作原理](/guide-moderation.html#moderation-approvals) in the moderation guide.
- 在拥有大量版主的租户中，此触发器的频率很高。请有选择地订阅并相应地预算资源。

---