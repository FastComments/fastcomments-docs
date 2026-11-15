每笔费用，无论是套餐续订、Flex 月度使用还是套餐变更，都会生成一张发票。发票在支付后即可作为收据使用。

### 查找发票

打开 [Billing History](https://fastcomments.com/auth/my-account/billing-history)，或在 Billing Info 页面点击 `View Invoices`。表格中列出每张发票的日期、账单周期、发票编号、描述、总额、状态（`Paid`、`Unpaid`、`Draft` 或 `Void`）以及到期日，每页显示十条，最新的排在最前面。

对于每张发票：

- `View` 会在 Stripe 托管页面上打开发票，您也可以在该页面下载发票。
- `PDF` 直接下载 PDF 文件。
- `Pay Now` 会代替 `View` 出现在未支付的发票上，并带您前往该发票在 Stripe 的支付页面，您可以在那里直接付款。

发票在创建时也会通过电子邮件发送到您的账单邮箱。如果希望发票发送到财务邮箱而不是账户所有者，请在 [Invoice Settings](#invoice-settings) 中设置 `Billing Email`。

### 发票上包含的内容

发票显示 [Invoice Settings](#invoice-settings) 中的账单名称、地址和 VAT / Tax ID，以及明细项目、已应用的任何预付[额度](#credits)和向您的银行卡收取的金额。Flex 发票会为每种使用类型列出单独一行，参见 [How Flex Usage Is Measured](#how-flex-usage-is-measured)。对于位于欧盟的账单地址，Flex 发票以 0% VAT 开具，页脚显示 `Reverse Charge Transaction: Customer to pay VAT`。

对发票设置的更改仅适用于保存之后创建的发票。已开具的发票无法编辑，因此请在下一个账单日期之前设置好您的公司信息和 VAT / Tax ID。

### 在其他地方计费的账户

如果您的账单由外部处理，例如通过与支持团队协商的发票方式，发票会发送给您进行支付，而不是向银行卡扣款，并且可能不会显示在 Billing History 中。通过 Shopify 或 BigCommerce 安装的账户由相应平台计费，Billing Info 页面会链接到该平台上的套餐页面。通过父账户计费的子账户没有自己的发票；参见 [When You Are Charged](#when-you-are-charged)。