没有单独的“编辑卡片”页面。要替换已保存的卡片，请打开 [Billing Info](https://fastcomments.com/auth/my-account/billing-info)，保持当前套餐不变，在 `Payment method` 下输入新卡信息，然后点击 `Save Info`。

新卡将成为之后所有账单的默认支付方式。保存时不会产生任何扣费，除非您有未支付的账单——在这种情况下，该账单将使用新卡收取：固定套餐会立即扣费，Flex 套餐则在一夜之间完成。当您的账单状态正常时，表单会显示 `You won't be charged now unless you change plans.` 如果您的固定套餐订阅已结束，保存卡片将开启新的订阅并收取第一个周期的费用，与首次注册时相同。

如果您的银行要求确认卡片，点击保存后会弹出确认窗口。只有银行批准后，卡片才会被保存。

### 谁可以执行此操作

只有 `Account Owner` 或拥有 `Billing Admin` 权限的用户才能保存卡片或更改套餐。其他用户会看到 `Only the account owner or a billing admin can change the plan or payment method.` 如果 `Settings` 菜单呈灰色或账单页面将您重定向到主页，说明您是以评论者或没有账单访问权限的用户身份登录，而不是账户所有者。请使用创建该网站的账户登录，或让所有者授予您相应权限；参见 [Who Can Manage Billing](#who-can-manage-billing)。

### 卡片被拒或被拦截

如果卡片被拒绝，页面会显示 `Your card was declined. Check that the entered information is correct.`，且不会保存任何内容。请检查卡号、有效期、CVC 和账单地址，然后重试。如果银行拒绝了确认，您会看到 `Your bank did not approve the payment. Try again, or use a different card.`

有些银行会拦截在线循环扣款，或在一次扣款失败后拦截该卡。在这种情况下，请尝试其他卡片，或在 [Billing History](https://fastcomments.com/auth/my-account/billing-history) 中使用 `Pay Now` 支付未结账单。如果您的银行完全不允许自动扣款，请[联系支持团队](https://fastcomments.com/auth/my-account/help)，我们可以改为发送账单的支付链接。

仅接受银行卡。表单中的 `Country` 列表包含 Stripe 可以扣款的国家/地区；如果缺少您的国家/地区，请联系支持团队。