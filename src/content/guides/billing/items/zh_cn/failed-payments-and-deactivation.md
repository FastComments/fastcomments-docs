付款失败的原因可能是卡片过期、被更换、余额不足，或银行拒绝了扣款。FastComments 会给你时间在一切停止工作之前解决问题，并通知能够解决它的人。

### 会发生什么

1. **扣款失败。** 发票在 [Billing History](https://fastcomments.com/auth/my-account/billing-history) 中保持 `Unpaid` 状态，并带有 `Pay Now` 按钮。在固定套餐上，Stripe 可能会重试扣款。在 Flex 上，FastComments 每晚都会重试任何超过宽限期的未支付发票。
2. **发送警告。** 每晚 FastComments 会检查每个账户。如果 Flex 发票超过宽限期仍未支付且重试失败，或者固定套餐订阅因无法收款而终止，则每个拥有 `Billing Admin` 权限的用户都会收到邮件 `Your FastComments Tenant Will Be Deactivated Soon!`，其中包含日期，并且账户被安排在一个宽限期后停用。在此期间，评论组件仍可正常工作。
3. **停用。** 如果到计划日期一切都没有改变，账户将被停用：你网站上的评论将被隐藏，访客会看到 `This FastComments account's billing information is more than sixty days out of date. Comments hidden until the issue is resolved.`。账单管理员会再次收到邮件。你的数据会被保留。

宽限期对大多数账户为一周，对超过一年的账户为两周。在 Flex 上，宽限期适用两次：一次在警告之前，一次在警告与停用之间，因此从扣款失败到评论被隐藏之间至少有两周时间。

### 如何解决

在 [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 页面保存一张可用的卡片。仅此一步即可立即重新激活账户并取消任何计划中的停用；未支付的发票随后会用新卡收款——固定套餐上立即收款，Flex 上在每晚检查时收款。你也可以直接在 Billing History 中使用 `Pay Now` 立即支付发票。

如果存档的卡片没有问题且失败是暂时的，你无需做任何事情：每晚检查会注意到已支付的发票或有效的订阅，取消停用，并向账单管理员发送确认邮件。已停用的账户也会以同样的方式重新激活。

### 多张发票同时出现

如果你的卡片在几个月内无效然后被修复，每个未支付的月份会分别收款，因此你可能会在同一天看到多笔扣款。请在 Billing History 中查看每张发票的账单周期：它们对应不同的月份，而不是重复扣款。

### 付款后仍看到账单警告？

警告属于特定账户。如果你已付款但某个网站仍显示 `billing information is out of date`，该网站上的评论组件很可能使用的是与你付款账户不同的账户。这种情况发生在网站被设置了两次，或者一个账户在 `fastcomments.com` 上而另一个在欧盟部署 `eu.fastcomments.com` 上时，两者是分开计费的。请将页面上代码中的 `tenantId` 与 `Settings -> API/SSO` 下的进行对比，或者[联系支持](https://fastcomments.com/auth/my-account/help)，我们会帮你理清。