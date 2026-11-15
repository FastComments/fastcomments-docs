Flex 账单是一份按用量类型逐项列出的清单，每项都显示数量和单价，例如 `213,000 Page Loads. ($1.00 per 100,000 Page Loads)`。费率见[定价页面](https://fastcomments.com/traffic-pricing)和您的[账单信息](https://fastcomments.com/auth/my-account/billing-info)页面。本节解释每个数量的含义。

### 按月统计

这些指标每月重置，从日历月的第一天到最后一天（UTC）进行统计：

- **Page Loads（页面加载）**：评论组件或信息流在您的页面上加载的次数。评论组件每次加载都会计数，无论它显示多少条评论。不按评论线程或页面单独收费。
- **API Credits（API 点数）**：使用您的 API 密钥调用 REST API 的次数。
- **Small Widgets Credits（小组件点数）**：小组件的加载次数，例如评论计数和徽章。
- **LLM Tokens（LLM 令牌）**：[AI Agents](/guide-ai-agents.html) 和其他 LLM 功能使用的令牌数量。

### 月末统计

这些是创建账单时获取的快照：

- **Comments Stored（已存储评论）**：您账户中的评论总数，而不仅仅是本月新增的评论。删除评论会减少该数量，但被删除且带有回复的评论会保留为占位符，以便回复仍然有意义。
- **SSO Users（SSO 用户）**：您的网站创建的 SSO 用户总数，包括很久以前只评论过一次的用户。如果这是您账单中占比最大的部分，请咨询客服关于按月活跃用户计费的方式，该方式只统计当月登录过的用户。
- **Moderators（版主）**、**Admins（管理员）**、**SSO Moderators（SSO 版主）** 和 **SSO Admins（SSO 管理员）**：席位，详见[管理员和版主席位](#admin-and-moderator-seats)。第一个管理员席位免费。
- **Domains（域名）**：您账户中的网站。第一个网站免费，每增加一个网站按月计费，只要它保留在账户中就会持续计费。子域名只有在您将其作为独立网站添加时才计数；一个顶级域名已涵盖其子域名。参见[多站点](/guide-multiple-sites.html#domain-limits)。
- **Managed Tenants（托管租户）**：由此账户管理的子账户（如有），以及它们各自的用量（作为单独的条目列出）。

### 最低消费

如果各项加起来低于 Flex 最低消费金额，账单将补足至最低金额。只有一个管理员和一个域名的小型网站只需支付最低金额。

### 预览账单

[账单分析](https://fastcomments.com/auth/my-account/analytics/billing)页面显示下一份账单的每一项及当天的数字，因此不会有意外。页面加载、API 和小组件的计数可能延迟最多一分钟；其余数据是实时的。

### 其他货币

Flex 用量以美元计价。如果您的账单货币是其他币种，每一项将按账单创建当天的汇率进行换算，因此相同的用量在不同月份的费用可能略有不同。参见[货币](#currencies)。