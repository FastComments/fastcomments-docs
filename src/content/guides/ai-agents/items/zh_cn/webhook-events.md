有四种代理（agent）Webhook 事件类型。每个事件都有一个数字枚举值（在有效载荷中使用）和一个规范的字符串名称（在 `event` 信封字段和 `X-FastComments-Agent-Event` HTTP 头中使用）。

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | 运行以 `SUCCESS` 状态完成。 |
| `trigger.failed` | 1 | 运行以 `ERROR` 状态完成。 |
| `approval.requested` | 2 | 一个审批被排队为 `PENDING` 状态。 |
| `approval.decided` | 3 | 一个审批转换为 `APPROVED`、`REJECTED` 或 `EXECUTION_FAILED`。 |

### `trigger.succeeded`

在代理运行无错误完成后触发。有效载荷的 `data` 字段包括：

- `triggerId` - 唯一的运行 ID。
- `triggerType` - 启动该运行的 [trigger reason enum](#triggers-overview)。
- `status` - `SUCCESS`（字符串）。
- `tokensUsed` - 此次运行消耗的 token 数量。
- `wasDryRun` - 如果代理处于 [dry-run 模式](#dry-run-mode) 则为 true。
- `actions` - `TenantAgentAction` 记录的数组（参见 [Webhook 有效载荷](#webhook-payloads)）。
- `commentId`, `url`, `urlId` - 如果触发器包含这些字段。

如果运行没有采取任何操作，`actions` 数组为空——这是一次成功的“代理决定不采取任何操作”的运行，这一点很有参考价值。

### `trigger.failed`

在运行出错时触发。有效载荷格式与 `trigger.succeeded` 相同，但 `status: 'ERROR'`，并增加了一个描述出错原因的 `errorMessage` 字段。可能的错误包括 LLM 调用失败、工具派发失败以及运行中预算耗尽。

`actions` 仍可能包含在错误发生前已完成的工具调用条目。

### `approval.requested`

在审批被排队为 `PENDING` 状态的那一刻触发。有效载荷包括：

- `approvalId`, `triggerId`。
- `toolName`, `actionType`。
- `status: 'PENDING'`。
- `args` - 工具的参数 **按原样传递** 自 LLM 调用。参数的结构依赖于具体工具，不是稳定的公共合约——随着新工具的添加，模式可能会变化。
- `createdAt`。
- `justification`, `confidence` - 如果代理提供了它们。
- `contextSnapshot` - 与该审批相关的评论/页面上下文。

适用于将待定审批转发到聊天运维通道：订阅 `approval.requested` 的 Slack 机器人可以将该操作和理由发布到审核频道，便于一目了然地审查。

### `approval.decided`

当审批不再处于 `PENDING` 时触发。有效载荷包括：

- `approvalId`, `triggerId`。
- `toolName`, `actionType`。
- `status` - `APPROVED`、`REJECTED` 或 `EXECUTION_FAILED`。
- `decidedBy` - 做出决定的审核者的用户 ID。
- `decidedAt` - 做出决定的时间。
- `executedAt` - 如果 APPROVED，则平台执行获批操作的时间。
- `executionResult` - 如果 APPROVED，为描述执行器结果的字符串。
- `contextSnapshot` - 评论/页面上下文。

该事件涵盖所有决策结果：

- **Approved + executed cleanly** -> `status: APPROVED`，`executedAt` 已设置，`executionResult` 为成功消息。
- **Approved + executor failed** -> `status: EXECUTION_FAILED`，`executedAt` 已设置，`executionResult` 描述失败原因。
- **Rejected** -> `status: REJECTED`，`executedAt` 为 null，`executionResult` 为 null。

### 头部

每次投递都会包含一个 `X-FastComments-Agent-Event` HTTP 头，值为事件的规范字符串名称（例如 `trigger.succeeded`）。如果你的端点是单个 URL 处理多种事件类型，这一点很有用。

### 另请参阅

- [Webhook 有效载荷](#webhook-payloads) 获取每个事件的完整有效载荷模式。
- [Webhook 签名](#webhook-signing) 获取 HMAC 方案。
- [Webhook 重试](#webhook-retries) 获取投递语义。