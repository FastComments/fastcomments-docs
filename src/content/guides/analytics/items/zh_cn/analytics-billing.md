您可能被限速或计费的任何指标都会在[计费分析页面](https://fastcomments.com/auth/my-account/analytics/billing)中报告。

以下是一些被跟踪的指标示例：

- 页面加载量（当月）
- API 积分（当月）
- 新存储的评论（当月）
- 租户用户
- SSO 用户（没有管理员/版主权限的普通 SSO 用户）
- SSO 管理员（带有 `isAccountOwner` 或 `isAdminAdmin` 标志的 SSO 用户 - 与普通 SSO 用户分开计费）
- SSO 版主（带有 `isCommentModeratorAdmin` 标志的 SSO 用户 - 与普通 SSO 用户分开计费）
- 版主
- 域名

对于 `Creators` 和 `Pro` 套餐，这些指标仅作为参考，帮助您了解是否接近或已达到账户限额。

对于 `Flex` 套餐，此页面将显示**下一期**账单的当前使用费用明细。这使您可以在账单产生之前轻松了解计费方式。您可以在[计费历史](https://fastcomments.com/auth/my-account/billing-history)中查看过往账单。每行费用的计算方式以及账单的扣费时间在[计费指南](/guide-billing.html#how-flex-usage-is-measured)中有说明。

`Page Loads`、`API Credits` 和 `New Comments Stored` 这些指标可能存在最多一分钟的延迟。其他指标为实时数据。

出于历史记录目的，此信息按月存储一年，或在您删除租户之前一直保留，但界面仅显示当前周期。