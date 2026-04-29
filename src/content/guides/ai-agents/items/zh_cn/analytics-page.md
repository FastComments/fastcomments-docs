Analytics 是跨代理仪表板。可从 AI Agents 页面通过 **Analytics** 选项卡（租户范围）访问，或通过每个代理行上的 **Analytics** 按钮按代理访问。

### Filter

顶部的下拉菜单 - **All agents** 或特定代理。页面其余部分将相应调整范围。

### Budget usage

四个进度条显示当前周期支出与上限的对比：

- **Agent today**（当筛选为特定代理时）- 每日代理上限。
- **Agent this month** - 每月代理上限。
- **Account today** - 租户每日上限。
- **Account this month** - 租户每月上限。

当未设置上限时，进度条显示“(no cap set)”并显示原始支出。

### Daily cost (last 30 days)

按所选范围以您租户的货币显示的每日成本表（最近 30 天）。有助于发现：

- **突发成本飙升** - 通常由失控循环或某条病毒式传播的评论引发大量触发造成。
- **成本漂移** - 随着社区增长，日成本逐渐上升。

### Actions taken

本月内按动作类型的分解 - “Wrote a comment: 47”、“Marked a comment as spam: 12”等。用于检查代理是否按预期工作。

### Triggers skipped (this month)

按[丢弃原因](#drop-reasons)分组的计数：

- 超过代理每日 / 代理每月 / 账户每日 / 账户每月 上限。
- 被限流。
- 并发饱和。

如果在此处看到丢弃，说明您的代理触及了预算或速率限制，错过了本应执行的触发。查看[丢弃原因](#drop-reasons)。

### Dry-run vs live (this month)

- **Enabled runs** - 本月执行了实际操作的运行次数。
- **Dry runs** - 本月处于模拟运行模式的运行次数。

一个有用的调整信号：尚未提升为已启用（Enabled）的全新代理将仅显示模拟运行。处于已启用且在此部分所有计数均为零的代理处于闲置状态——要么未被触发，要么被范围排除，要么其触发配置不正确。

### Top agents by monthly cost

当筛选为 **All agents** 时，页面按本月到目前为止的成本对代理进行排名。发现最昂贵的代理是成本优化的第一步——通常的解决办法是“收紧其[context options](#context-options)”或“降低其[budget cap](#budgets-overview)”。

### Agents at or near their cap

按代理分解列出在当前期间支出达到或接近其每代理上限的代理：

- **near cap** - 支出超过上限的可配置百分比。
- **over cap** - 实际被限制，且在该期间有 `{count} dropped` 触发被丢弃。

从此表中点击进入代理，可提高上限、缩小范围或暂停该代理。

### Account summary

当筛选为 **All agents** 时：

- **Triggers today** - 次数。
- **Triggers this month** - 次数。
- 对于每项：显示跳过数量的 `dropped` 后缀。

### Currency

所有货币数值以您租户的货币显示。

### What this page does not do

- 它不显示 **每项操作的成本分解**——这些位于[运行详细视图](#run-detail-view)。
- 它不显示 **记录** 或 **LLM 响应**。
- 它不允许您对代理采取操作——编辑、暂停、删除均在代理列表/编辑页面完成。