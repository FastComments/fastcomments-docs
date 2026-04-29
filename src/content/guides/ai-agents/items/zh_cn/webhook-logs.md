每个 agent webhook 都有自己的投递日志。可从 [webhook 列表页面](https://fastcomments.com/auth/my-account/ai-agents/webhooks) 通过每行 webhook 的 **日志** 按钮访问。

### 页面内容

一个分页表格，每次投递尝试一行：

| Column | Meaning |
|---|---|
| When | 发生尝试的时间。 |
| Event | 事件类型（`trigger.succeeded`, `approval.requested` 等）。 |
| Status | 投递状态。 |
| StatusCode | 你的端点返回的 HTTP 状态码（如果可用）。 |
| Description | 结果的简短描述。对于没有收到 HTTP 响应的失败投递，底层错误信息会以 `{error: <message>}` 的形式存储。 |

该页面仅支持分页——不提供状态、事件类型或日期范围过滤。

### 日志中可以做什么

- **决定某个状态码是否应加入“不重试”。** 如果你看到你的端点反复返回相同的 `4xx`，那通常意味着你应将其添加到 **不重试状态码**，以便平台停止重试。

### 失败信息

当投递失败且没有 HTTP 响应（DNS 失败、连接被拒绝、超时、TLS 错误等）时，原始错误消息会以 `{error: <message>}` 记录。平台不会将这些错误归类为诸如 `TIMEOUT` 或 `DNS_ERROR` 之类的命名桶——直接读取错误消息以查看发生了什么。

对于 HTTP 响应，StatusCode 列显示你的端点返回的代码。常见情况：

- **所有尝试均为：`401` 或 `403`** - 你的端点在拒绝签名。检查你是否对 `${timestamp}.${body}` 计算 HMAC 并使用了正确的 secret。见 [Webhook Signing](#webhook-signing)。
- **所有尝试均为：`422`** - 你的端点认为负载无效。要么修复你的端点，要么如果某些事件预期会被拒绝，则将 `422` 添加到 **不重试状态码**。

### 每次投递的上下文

每个日志条目包含：

- `webhookId` - 哪个 webhook 配置产生了此投递。
- `agentId` - 此投递所涉及的 agent。
- `triggerId` 或 `approvalId` - 底层记录。
- `domain` - 匹配的域名。

你可以使用这些信息将失败的投递与 [运行历史](#run-history) 中相关的运行进行关联。

### 保留期限

`AgentSyncLog` 条目在 `createdAt` 上有统一的一年 TTL，无论结果如何——成功和失败的投递都会保留相同的时间。超过保留期后，日志条目将被删除。

如果你需要长期审计，推荐的做法是：让**端点本身**持久化它接收到的投递。这可以将你的审计日志与平台的保留策略解耦。

### 测试发送

webhook 配置表单的 **测试发送** 按钮会将一个伪造的投递写入相同的日志表，以便你在依赖真实事件之前验证端到端的连通性。测试投递在日志中会有明显标记，因此不会污染生产失败统计数据。

### 另见

- [Webhook 概览](#webhooks-overview)。
- 有关驱动这些日志的重试语义，请参阅 [Webhook Retries](#webhook-retries)。
- 有关如何在你的端点上验证，请参阅 [Webhook Signing](#webhook-signing)。