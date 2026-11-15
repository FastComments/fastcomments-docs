您的卡片何时被扣费、扣多少，取决于您所使用的套餐类型。

### 固定套餐

当您开始 `Creators` 或 `Pro` 订阅时，第一个月或第一年的费用会在您点击 `Pay Now & Start Subscription` 时立即在浏览器中扣取。如果您的银行要求您确认付款，请在弹出的窗口中完成验证。只有付款成功后，订阅才会开始。

此后，订阅将在每月的同一天续费（如果您选择了 `Pay Annually`，则为每年的同一天），从您订阅之日起计算。续费会自动从您存档的卡片中扣取，发票会发送到您的账单邮箱。您无需进行任何操作。

按年计费比按月计费便宜 15%。Flex 无法按年计费。

### Flex

Flex 按后付费方式计费：您在一个日历月内使用服务，该月的发票会在当月最后一天（UTC 时间）生成，并立即从您的卡片中扣取。账单日期始终是当月最后一天，无法更改。

发票仅涵盖该日历月。您注册当月的用量不会被计费，因此您的第一张发票最早会在注册后的下一个月末生成。

如果您的用量低于 Flex 最低消费额，发票上会出现一行 `FastComments Flex. (minimum ... /mo)`，将金额补足至最低消费额。当前的最低消费额显示在您的 [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 页面和[定价页面](https://fastcomments.com/traffic-pricing)上。

您可以随时查看下一张发票的金额。`Next Invoice Amount` 显示在 [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 页面上，完整的逐项明细显示在 [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) 页面上。计费内容的说明请参阅 [How Flex Usage Is Measured](#how-flex-usage-is-measured)。

### 保存卡片不会扣费

账单设置完成后，在不更改套餐的情况下保存或更新卡片绝不会扣费，除非您有未支付的发票。表单会显示 `You won't be charged now unless you change plans.`，卡片只是存储在 Stripe 中以供下一张发票使用。只有当您尚未设置有效账单时，表单才会显示 `Your card will be charged now.`，即便如此，Flex 账户也要到月底才会被扣费。您账户中的任何预付[额度](#credits)都会在扣卡之前先应用于每张发票。

### 托管子账户

如果您的账户管理子账户（例如每个站点或环境一个，或白标租户），则只有父账户会被计费。每个子账户的用量会作为单独的行显示在父账户的 Flex 发票上，并带有子账户名称前缀，子账户共享父账户的付款方式和账单状态。