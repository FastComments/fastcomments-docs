Credits 頁面底部的 `Past top-ups` 表格會列出您最近的儲值記錄，由新到舊排列，並顯示每筆儲值的 `Date`、`Amount` 和 `Status`。在您首次儲值之前，表格會顯示 `No top-ups yet.`

每筆儲值會有以下三種狀態之一：

- `Credited`：付款成功，金額已加入您的餘額。
- `Pending`：付款已開始但尚未完成，通常是因為銀行確認手續未完成。待處理的儲值會在下次開啟 Credits 頁面時與 Stripe 核對，並轉為 `Credited` 或 `Failed`。
- `Failed`：信用卡被拒絕或銀行未核准付款。沒有產生任何扣款，您的餘額也未變動。

此表格記錄的是從本頁面進行的儲值。以儲值餘額支付的發票會顯示在發票本身，即 [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 頁面的 `View Invoices` 下方。