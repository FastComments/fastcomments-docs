Credits 页面底部的 `Past top-ups` 表格列出了您最近的充值记录，按时间从新到旧排列，包含每笔充值的 `Date`、`Amount` 和 `Status`。在您首次充值之前，该表格显示 `No top-ups yet.`

每笔充值具有以下三种状态之一：

- `Credited`：支付成功，金额已添加到您的余额中。
- `Pending`：支付已开始但尚未完成，通常是因为银行确认未完成。待处理的充值会在下次打开 Credits 页面时与 Stripe 进行核对，并转为 `Credited` 或 `Failed`。
- `Failed`：银行卡被拒绝或银行未批准该笔支付。未产生任何扣款，您的余额也没有变化。

该表格记录从此页面发起的充值。使用余额支付的发票会显示在发票列表中，位于 [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 页面的 `View Invoices` 下方。