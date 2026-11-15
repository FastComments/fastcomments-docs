在 Flex 计划中，能够管理或审核您账户的用户将按月度席位计费。这是 Flex 账单中最容易让人感到意外的一部分，通常是因为席位在无人察觉的情况下被创建了。

### 什么算作一个席位

- **管理员**：您账户的 `Users` 页面上拥有任何管理权限（计费、自定义、分析、审核等）的用户。第一个管理员（通常是账户所有者）是免费的；每增加一个管理员就是一个席位。
- **审核员**：`Moderators` 页面上已接受邀请的用户。待处理的邀请不计费。仅有评论审核权限的用户按审核员计费，而不是管理员。
- **SSO 管理员**和 **SSO 审核员**：您的网站通过在 SSO 载荷中设置 `isAccountOwner`、`isAdminAdmin` 或 `isCommentModeratorAdmin` 来登录的 SSO 用户。他们的计费费率与普通管理员和审核员相同。参见 [SSO User Structure](/guide-api.html#sso-user-structure)。

席位按电子邮件地址去重，因此同一个人如果既是管理员又是审核员，或者同时拥有相同电子邮件的 SSO 用户和仪表板登录，则只按较高的费率计费一次。拥有两个不同电子邮件地址的人则是两个席位。

在按月活跃用户计费的账户上，当月未登录的 SSO 审核员在该月不计费。

### 您未主动添加的席位

有两个功能会自动创建席位：

- **SSO 管理员同步。** 当 SSO 用户使用与您某个仪表板用户匹配的电子邮件登录时，该用户的管理员或审核员权限会被复制到 SSO 用户。使用 WordPress SSO 时，这意味着每个 WordPress 管理员都可以成为 FastComments 的管理员或审核员。您可以在 `Customize` 下的自定义规则中使用 `Disable Automatic Admin Migration` 选项将其关闭。
- **同一个人使用不同的电子邮件。** 如果您的仪表板登录使用一个电子邮件，而您的 SSO 用户使用另一个，则它们是两个席位。在两者上使用相同的电子邮件即可将它们合并。

### 检查和移除席位

[Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) 页面列出了您的 `Moderators`、`SSO Admins` 和 `SSO Moderators` 数量，下一张发票预览会显示它们的费用。您可以从 `Users` 和 `Moderators` 页面移除不需要的用户，或从 SSO 载荷中删除管理员标志。席位在月底创建发票时统计，因此在此之前移除的席位在该月不计费。

有关如何管理两者的信息，请参见 [Adding Moderators](/guide-users.html#adding-moderators) 和 [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions)。