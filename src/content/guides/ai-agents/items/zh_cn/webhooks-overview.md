Agent webhooks 是平台在代理运行完成或审批状态发生变化时触发的 HTTP 回调。将它们用于将代理活动转发到您自己的系统——审核仪表板、审计日志、Slack 频道、升级工具等。

在 [AI Agents 页面](https://fastcomments.com/auth/my-account/ai-agents) 的 **Webhooks** 选项卡下进行配置。

### 会传递哪些内容

四种事件类型：

- **`trigger.succeeded`** - 代理运行成功完成。
- **`trigger.failed`** - 代理运行发生错误。
- **`approval.requested`** - 一个操作被排队等待人工审批。
- **`approval.decided`** - 一个审批被批准、拒绝或执行失败。

参见 [Webhook Events](#webhook-events) 了解哪些事件在何时触发，参见 [Webhook Payloads](#webhook-payloads) 获取每个事件的模式。

### 不会传递的内容

- **按工具操作的单独 webhook。** 调用 `pin_comment` 的运行不会为该固定操作触发单独的 webhook——该操作包含在运行的 `trigger.succeeded` 有效载荷中。如果您想要按操作交付，请解析触发器有效载荷中的 `actions` 数组。
- **被丢弃的触发器。** 未调度的触发器（超出预算、范围错误）不会触发 webhook。丢弃仅在 [Analytics page](#analytics-page) 可见。
- **重放产生的触发器。** 测试运行不会触发 webhook。参见 [Test Runs (Replays)](#test-runs-replays)。

### 配置

每个 webhook 配置项包括：

- **URL** - 要 POST 的 HTTPS 端点。
- **Domain** - 此 webhook 适用的评论域（您的租户可能托管多个域）。`*` 匹配所有域；`*.example.com` 是一个通配；精确域仅匹配该域本身。
- **Events** - 要订阅的四种事件类型中的哪些。
- **Agents** - 留空表示“所有代理”，或列出要过滤的特定代理 ID 列表。
- **Method** - POST 或 PUT（默认 POST）。
- **No-retry status codes** - 应视为终止性失败且不重试的 HTTP 响应代码（例如 410 Gone、422 Unprocessable）。参见 [Webhook Retries](#webhook-retries)。

多个 webhook 可以订阅相同的事件——每个 webhook 独立入队并交付到其各自的 URL。

### 按域匹配

给定事件会交付到**其 `domain` 字段匹配评论域的每个 webhook**。匹配使用简单的通配符规则：

- 精确：`example.com` 仅匹配 `example.com`。
- 通配星号：`*` 匹配所有域。
- 子域通配：`*.example.com` 匹配 `blog.example.com`、`forum.example.com`，但不匹配 `example.com` 本身。

有效载荷上的域为以下项中第一个非空结果：评论的 `domain`、基于您租户域配置解析的 URL，或通过 `urlId` 查找的 `Page`。

### 按代理过滤

**Agents** 字段允许 webhook 仅订阅某些代理。留空表示“所有代理”。当非空时，只有列表中的代理会触发该 webhook。

当您为审核事件和互动事件分别设置不同 webhook 并发送到不同下游系统时，这非常有用。

### 测试发送

webhook 配置 UI 有一个 **Test send** 按钮，会向该 URL 发布一个假的有效载荷，以便您在不等待真实事件的情况下验证连通性、签名和端点的响应代码。

### 传递日志

每次投递（以及每次重试）都会记录在 [Webhook Delivery Logs](#webhook-logs) 页面，这样您就可以查看线上的实际传输情况。

### 签名

每个 webhook 都使用租户的 API 密钥通过 HMAC-SHA256 进行签名。参见 [Webhook Signing](#webhook-signing)。

### 可用性

Agent webhooks 需要租户具备有效计费。它们使用与现有评论 webhook 相同的签名/密钥基础设施——如果您已集成评论 webhook（参见 [Webhooks guide](/guide-webhooks.html)），则代理 webhook 的集成方式相同，只是包含了新的事件类型。

---