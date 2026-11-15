计费仅限于两类用户：

- `Account Owner`，即创建账户的用户。
- 任何拥有 `Billing Admin` 权限的用户。

只有这些用户可以更改套餐、保存或更换卡片、打开 `Invoice Settings`、设置 Flex 消费限额以及添加信用额度。其他任何尝试的用户都会看到 `Only the account owner or a billing admin can change the plan or payment method.`

计费相关邮件，例如试用期中途提醒、付款失败警告和停用通知，会发送给拥有 `Billing Admin` 权限的用户。请将该权限授予应当接收这些邮件的人员，例如财务人员，他们无需对仪表板拥有任何其他访问权限即可收到邮件。试用结束后未填写计费信息而发送的提醒会发送给账户上的每一位仪表板用户。这些邮件中的每一封都包含退订链接；选择退出账户通知的用户将不再收到这些邮件。

发票本身会发送至 [Invoice Settings](#invoice-settings) 中的 `Billing Email`，该邮箱完全不必属于任何仪表板用户。

[Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) 页面则需要 `Analytics Admin` 权限，因为它显示的是使用情况而非付款详情。

要授予或移除该权限，请打开 `Users` 页面并编辑该用户。参见 [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions)。在 Flex 上，请注意每增加一名管理员都会计为一个付费席位，参见 [Admin and Moderator Seats](#admin-and-moderator-seats)。

如果您已登录但完全看不到 `Billing Info`，或者它将您跳转回主页，说明您是以评论者账户而非仪表板用户身份登录的。请退出登录，并使用拥有该网站的账户重新登录。