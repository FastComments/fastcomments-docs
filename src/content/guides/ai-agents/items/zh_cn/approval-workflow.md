一个**批准（approval）**是一个排队的工具调用，平台在执行前需要人工批准或拒绝。

### 配置批准

在代理编辑表单中，**Approvals** 部分列出代理被允许调用的每个工具（允许列表），并让你勾选那些必须由人工审查的工具。未勾选的工具会立即执行。勾选的工具会被排队。

被禁止的工具会*被直接拒绝*，不会排队——批准仅适用于那些本来被允许的工具。

### 当受限操作触发时会发生什么

1. 代理选择一个工具调用（例如 `ban_user`）及其参数、理由和置信度。
2. 平台不会执行，而是将批准以 `PENDING` 状态排队，包含工具名称、参数、理由、置信度和描述触发该操作的上下文快照。
3. 通知会发送给审核者（参见 [Approval Notifications](#approval-notifications)）。
4. 代理的运行完成并被记录——该操作在 [Run Detail View](#run-detail-view) 中显示为 **Pending approval**。

### 审核批准

在 [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) 的批准收件箱列出待处理、已批准、已拒绝和执行失败的批准。对于每个条目：

- **工具名称和参数** - 代理想要做的确切内容。
- **代理理由** - 代理提供的论证。
- **置信度** - 代理自评的置信度，范围 0.0 到 1.0。
- **上下文快照** - 操作所针对的评论、页面和用户。

两个按钮：**Approve** 和 **Reject**。Approve 会实际执行该工具；Reject 会丢弃该请求。

### 批准状态

批准会在以下状态之间移动：

| State | Meaning |
|---|---|
| `PENDING` | 等待人工决定。 |
| `APPROVED` | 人工批准并且操作已运行。 |
| `REJECTED` | 人工拒绝。该操作未运行。 |
| `EXECUTION_FAILED` | 人工批准但操作失败（例如目标评论已被删除）。 |
| `EXECUTING` | 瞬态状态：人工点击 Approve 后操作正在运行。用于序列化并发的批准点击，以确保具有真实副作用的工具不会运行两次。 |

当两个审阅者同时点击 Approve 时，`EXECUTING` 状态很重要——一个会成功，另一个会看到该批准已经移动。

### 会被清理的内容

待处理的批准会保持待处理状态直到有人决定。它们在创建后 **90 天** 自动过期。处于任何其他状态的批准也会在 90 天后为存储整洁而被清除。

批准的“决定者 / 决定时间 / 执行时间 / 执行结果”等字段会随着批准状态的变更而被填充——所有这些都可以在收件箱详情视图中查看。

### Webhook 集成

当批准状态变更时会触发两个 webhook 事件：

- **`approval.requested`** - 在插入为 PENDING 时触发。
- **`approval.decided`** - 在转换为 APPROVED、REJECTED 或 EXECUTION_FAILED 时触发。

两者的签名方式与其他 webhook 相同。参见 [Webhook Events](#webhook-events) 和 [Webhook Payloads](#webhook-payloads)。

### 加固：已知工具门控

批准拒绝将任何非已识别代理工具的工具名称排入队列。这是一种纵深防御检查：即使将来某个代码路径把来自 LLM 的工具名称传入批准流程，该字符串也永远无法作为排队批准落地。已知工具名称集合与 [Tools Reference](#tools-overview) 中列出的集合相同。

### 常见的门控模式

- **全新审核代理** - 对 `ban_user`、`mark_comment_spam`、`mark_comment_approved`、`send_email` 进行门控。观察收件箱几周，然后对低错误率的工具取消门控。
- **长期审核代理** - 对 `ban_user` 以及任何不可逆操作（`deleteAllUsersComments`、`banIP`）保持永久门控。
- **欧盟地区** - 无论你勾选什么，`ban_user` 都会根据第 17 条被锁定。参见 [EU DSA Article 17 Compliance](#eu-dsa-compliance)。

### 批准不会做的事

- 它们不会延迟代理的其他工具调用。代理的运行可以调用多个工具，只有受限的那些会排队——其余按正常执行。
- 如果人工拒绝，它们不会回滚代理的运行。运行中非受限的部分已经完成。