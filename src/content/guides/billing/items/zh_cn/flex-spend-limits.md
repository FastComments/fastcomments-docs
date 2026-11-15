Flex 没有固定的用量上限，因此流量激增或失控的集成可能会产生超出预期的账单。支出限制（Spend limits）可以让你对账单中基于用量的部分设置上限。

在 [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 页面上，套餐摘要会显示 `Monthly spend limit`。点击 `Edit` 打开对话框。你可以输入一个 `Target monthly max` 让 FastComments 自动设置各项限制以匹配该目标，也可以自行设置每一项限制：

- `Page loads per month`
- `Comments stored`
- `SSO users`
- `API credits per month`
- `Small widget credits per month`

将某个字段留空即可保持该类用量不受限制。对话框会显示你输入的限制所对应的 `Estimated monthly max`，且 Flex 的最低消费仍然适用。

### 限制的作用

当达到某项限制时，该类用量在当月剩余时间内将被阻止。对于页面加载而言，这意味着挂件将停止加载评论并显示 `This FastComments.com account has exceeded its limits.`，直到月份切换或你提高限制为止。提高后的限制会在几分钟内生效。

支出限制并不是对整个账单的硬性上限。它仅涵盖上述用量项目。你自行添加的席位和站点（例如版主或额外域名）无论如何都会被计费，因为这些由你直接控制。你可以随时在 [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) 上查看下一张账单的预览。

只有 `Account Owner` 或 `Billing Admin` 才能更改支出限制。