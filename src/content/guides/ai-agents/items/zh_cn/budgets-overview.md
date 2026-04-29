每个代理都有支出上限。达到上限时平台会停止调度该代理，周期重置后恢复。

### 两个范围，两个周期

总共有四个上限 —— 两个范围（每代理、每租户）与两个周期（每日、每月）的组合。

| Scope | Period | Where you set it |
|---|---|---|
| Per-agent daily | UTC day | Agent edit form -> **Budget** -> **Daily budget** |
| Per-agent monthly | calendar month | Agent edit form -> **Budget** -> **Monthly budget** |
| Per-tenant daily | UTC day | Plan-derived (no separate user-facing input) |
| Per-tenant monthly | calendar month | Plan-derived (no separate user-facing input) |

只有在**所有四个上限**都允许的情况下触发器才会被调度。最先耗尽的上限会导致触发器被丢弃。

### 货币

每个代理的预算以您的账户货币输入。

### 达到上限时会发生什么

- 触发器会被记录为**丢弃**，并带有一个像 `agentDaily` 或 `tenantMonthly` 这样的[丢弃原因](#drop-reasons)。
- 丢弃计数会显示在[Analytics page](#analytics-page)的“跳过的触发（本月）”下。
- 不会进行 LLM 调用；被丢弃的触发本身不会消耗令牌。
- 代理的状态保持不变——只是无法调度，直到周期重置。

### 周期重置

- **每日**上限在 UTC 午夜重置。
- **每月**上限在每个日历月的开始按 UTC 重置。

未使用的预算不会结转到下一个周期。

### 硬上限与软提示

上限是**严格的**。没有“超出 10% 并警告”的模式。当达到上限时，调度会停止。

“软” 的部分是 [Budget Alerts](#budget-alerts) 电子邮件 —— 在可配置的阈值（默认 80% 和 100%）会收到邮件，便于您在流量开始下降之前提升上限。

### 在哪里查看当前使用情况

- [Analytics page](#analytics-page) - 按代理和全租户的预算使用情况，带有上限标记。
- 代理编辑表单的 **Stats** 部分。
- 列表视图（代理卡片上显示待批准数量和最近运行次数）。

### 选择预算

一些经验法则：

- **新代理** - 确定预算。观察 [Run History](#run-history) 一周。根据观察到的每次运行成本 × 预期触发量进行调整。
- **高流量代理**（例如繁忙站点上的新评论触发） - 日上限是用于捕捉失控循环的。选择一个相当于预期日消费 2-3 倍的日上限，这样正常的繁忙日也能轻松在其下运行。
- **摘要器或上下文密集型代理** - 每次运行的成本较高。设置更严格的日上限以防止糟糕的一天耗尽月度预算。

### 重放的预算绕过

[测试运行 / 重放](#test-runs-replays) 受其**自身**的硬上限限制（在重放表单上设置，独立于代理的日/月上限），同时也受代理和租户上限的限制。先达到的上限会停止重放。

### 另见

- [Budget Alerts](#budget-alerts) 关于电子邮件通知的说明。
- [Cost Model](#cost-model) 说明平台如何将令牌转换为美元。
- [Drop Reasons](#drop-reasons) 列出触发器未运行的全部原因。