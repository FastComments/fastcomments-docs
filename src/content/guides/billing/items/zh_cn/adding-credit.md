充值将计入您账户中保存的支付方式。如果您尚未保存银行卡，Credits 页面会显示 `Add a payment method first` 以及一个 `Go to billing` 按钮。请先在 [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 页面保存一张银行卡，然后再返回。

添加余额的步骤：

1. 打开 [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) 并点击 `Manage Credits`。
2. 在 `Add credit` 下方，选择一个预设金额，或选择 `Other amount` 并输入您自己的金额。
3. 点击 `Add` 按钮。按钮会显示您选择的金额，例如 `Add $25.00`。
4. 如果您的银行要求您确认付款，请在弹出的窗口中完成验证。

付款成功后，页面会显示 `Credit added` 以及您的新余额，并且该充值会以 `Credited` 状态出现在 `Past top-ups` 中。点击 `Add more` 可再次充值。

### 金额与限额

预设金额为 `10`、`25`、`50` 和 `100`（以您的结算货币计）。自定义金额每次充值可在 `5` 到 `10,000` 之间，您的货币的确切限额会显示在表单下方。对于没有小数位的货币（例如日元），预设金额和限额会放大一百倍，以代表大致相同的金额。

充值始终使用您账户的结算货币，即与您的发票相同的货币。页面会告知您将使用哪种货币扣款。

### 银行确认（3D Secure）

某些银行要求您确认在线付款，通常通过您的银行应用或一次性验证码。如果您的银行有此要求，点击 `Add` 后会打开一个确认窗口。只有在银行批准后，您的银行卡才会被扣款，余额才会更新。

如果您在完成确认之前关闭了页面，该充值会被记录为 `Pending`。下次打开 Credits 页面时，我们会通过 Stripe 检查付款状态，并将其标记为 `Credited` 或 `Failed`。如果银行未批准付款，页面会显示 `Your bank did not approve the payment. Try again, or use a different card.`，并且不会产生任何扣款。

### 银行卡被拒

如果您的银行卡被拒绝，页面会显示 `Your card was declined. Check your card on the billing page, or try another card.` 不会产生扣款，您的余额不会改变，并且该次尝试会以 `Failed` 状态列在 `Past top-ups` 中。请在 [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 页面更新银行卡后重试。

为保护您的银行卡安全，每个账户每小时只能进行有限次数的充值尝试。如果您看到 `Too many top-up attempts. Wait a while and try again.`，请稍候再试。

付款由 Stripe 处理。FastComments 绝不存储您的银行卡信息。