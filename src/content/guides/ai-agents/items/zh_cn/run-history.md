运行历史是每个代理运行的每个触发器的日志。可从代理列表页面通过 **运行** 按钮访问，或直接在 `/auth/my-account/ai-agents/{agentId}/runs` 访问。

### 页面上有什么

一个分页表格，每次运行一行：

| 列 | 含义 |
|---|---|
| Date | 触发器触发的时间（或延迟触发器运行的时间）。 |
| Status | **进行中**、**成功** 或 **错误**。如果运行处于模拟运行模式，会同时显示 **模拟运行** 徽章。 |
| Cost | 以您租户货币计的每次运行费用。进行中（**进行中**）的运行此项为空。 |
| Actions | 该次运行中的工具调用次数。 |
| Details | 一个 **查看** 按钮，可打开 [运行详细视图](#run-detail-view)。 |

### 状态含义

- **进行中** - 该次运行正在进行，或在完成前中止。长时间停留在“进行中”通常表示 LLM 调用超时。
- **错误** - 运行已完成但在某处失败 - LLM 调用返回错误、工具调度失败等。详细视图包含具体错误信息。
- **成功** - 运行已完成且无错误。代理可能采取了零次、一次或多次操作。

### 空状态

当某个代理没有运行记录时，页面会显示：“No runs yet for this agent. Enabled runs appear here once a trigger fires; use Test run to preview what this agent would do against past comments.”

最后这句话是有意为之——推荐使用 [测试运行流程](#test-runs-replays) 在新代理上预先填充运行历史。

### 在运行历史页面上没有的内容

- **从未派发的实时触发器** - 因预算、范围或速率限制被丢弃的触发器不会出现在此页面。那些会在 [Analytics page](#analytics-page) 的 “Triggers skipped” 下显示。
- **审批** - 此次运行中采取的操作的待审批项位于 [审批收件箱](#approval-workflow)。该操作会在运行详细视图中显示为 **等待审批**。

### 保留

单条运行记录会保留 90 天，之后该运行将从历史记录中删除。费用和触发器计数仍会汇总到长期分析摘要中，因此 [Analytics page](#analytics-page) 仍会显示超出该时间窗口的历史总计。

### 回放

回放产生的运行默认不在实时运行视图中显示。你可以在 [测试运行（回放）](#test-runs-replays) 页面查看这些运行。

### 跨代理过滤

运行表是按代理划分的。没有跨代理的运行视图——[Analytics page](#analytics-page) 是跨代理的汇总。如果你需要检查多个代理的运行，可以将 [Webhooks](#webhooks-overview) 的 `trigger.succeeded` 和 `trigger.failed` 事件转发到你自己的系统。