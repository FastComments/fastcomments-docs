[发票设置](https://fastcomments.com/auth/my-account/invoice-settings)页面可通过账单信息中的 `Invoice Settings` 按钮进入，用于控制发票上显示的内容以及发票的发送位置。只有 `Account Owner` 或 `Billing Admin` 才能打开该页面。

- `Company Name`：您的账户名称，显示在仪表板、电子邮件和团队邀请中。这不是账单名称。
- `Billing Email`：发票和账单通知的发送地址。如果账户所有者不应接收这些通知，请将其设置为财务或账务邮箱。它不会更改任何用户的登录邮箱。
- `Invoice Currency`：发票的开具货币。参见[货币](#currencies)。
- `Billing Address`：打印在发票上的 `Name`、`Address`、`City`、`State / Province / Region`、`Postal Code` 和 `Country`。保存银行卡时，账单信息表单中也会显示相同的字段；两者写入同一个位置。
- `VAT / Tax ID`：可选。打印在您的发票上。

点击 `Save Changes`。页面会显示 `Invoice settings updated successfully.` 进行确认，更改将应用于下一张发票。已有的发票不会更改。

### VAT 和税务 ID

在输入 ID 之前，请先选择您的账单 `Country`，因为国家决定了 ID 的类型：欧盟成员国为欧盟 VAT 号码，英国为英国 VAT 号码，澳大利亚为 ABN，加拿大为 GST/HST 号码，美国为 EIN，Stripe 支持的其他国家依此类推。

ID 会在保存时进行验证。如果该 ID 对所选国家无效，页面会提示，且该 ID 不会被保存，而您的其他设置仍会保存。如果您的国家不在我们可以记录税务 ID 的范围内，页面也会告知您。

如果您的账单地址在欧盟，Flex 发票将按反向征收交易以 0% VAT 开具，您的 VAT 号码一旦保存就会显示在发票上，这样您的会计师可以在您这边处理 VAT。