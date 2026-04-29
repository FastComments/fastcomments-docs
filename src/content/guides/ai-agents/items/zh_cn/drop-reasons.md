当代理的触发器触发但**未**导致 LLM 调用时，平台会记录一个带有原因的“丢弃”。丢弃会显示在 [Analytics page](#analytics-page) 的 "触发器被跳过（本月）" 下。

### The full list of drop reasons

| Reason | What happened |
|---|---|
| `agentDaily` | 达到代理的每日预算上限。 |
| `agentMonthly` | 达到代理的每月预算上限。 |
| `tenantDaily` | 达到租户的每日预算上限。 |
| `tenantMonthly` | 达到租户的每月预算上限。 |
| `qps` | 命中代理的每分钟速率限制（滚动 60 秒窗口）。 |
| `concurrency` | 代理的最大并发运行数已被耗尽。 |

### What's not in this list

未进入调度路径的触发器不会被标记为带有原因的“丢弃”——它们只是未被调度。包括以下情况：

- 代理被 **禁用**。
- 触发的评论不匹配代理的 [URL/locale scope](#scope-url-locale)。
- 触发操作由同一代理执行（用于防止循环）。
- 租户的计费无效。
- 该代理不在租户的计划中。

这些是静默跳过，而非丢弃。它们不会出现在分析中的丢弃图表中。

### Reading drops on Analytics

[Analytics page](#analytics-page) 显示：

- **触发器被跳过（本月）** - 按丢弃原因分组计数。
- **达到或接近上限的代理** - 每个代理的细分，显示哪些代理正在接近上限，并列出当前期间被丢弃的触发器数量。

### What to do when you see drops

- **`agentDaily` / `agentMonthly`** - 代理自身的配额过于严格。可以在编辑表单中提高配额，或缩小代理的范围（URL/locale、更窄的触发条件）。
- **`tenantDaily` / `tenantMonthly`** - 账户级配额过紧。在租户计费设置中提高，或将消耗分配到更少的代理上。
- **`qps`** - 流量达到了每分钟滚动窗口限制。通常表明某个病毒性贴子快速扩散，触发速度超过代理的处理能力。代理的 `maxTriggersPerMinute` 和 `maxConcurrent` 字段会限制这一点；增加它们会提高吞吐量，但也会增加突发成本。
- **`concurrency`** - 与 `qps` 根本原因相同，但体现在进行中的并发数量上。如果需要更多并行性，请提高 `maxConcurrent`。

### Drops vs errors

丢弃表示“触发器从未运行”。而**错误**表示“触发器已运行，但 LLM 调用或工具分发失败”。错误在 [Run History](#run-history) 页面上单独跟踪（状态为 `Error`）。

### Drops can also stop replays

相同的丢弃原因也会停止正在进行的[测试运行/重放](#test-runs-replays)。重放会以错误状态停止，并给出一条指明哪个配额被触及的消息（例如，代理的每日配额）。

### Loop prevention is silent on purpose

不会为“此触发来自另一个代理并为防止循环而被跳过”记录丢弃原因。记录此类信息只会淹没分析数据而没有有用信号——按设计，代理的扇出不应导致触发爆炸。如果你怀疑某个本应运行的循环被错误抑制，请检查 [Comment Logs](/guide-moderation.html#comment-logs) —— 由机器人撰写的评论上的 `botId` 是循环检查所依据的键。