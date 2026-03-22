使用 FastComments 禁止用户在您的网站上发表评论有两种方式。

第一种是如果您已经知道他们的电子邮件，可以在 <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">封禁用户</a> 页面输入。

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

此页面可通过 审核评论 -> 封禁用户 访问

当我们要封禁用户时，我们可以选择一种类型，永久封禁或永久影子封禁：

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

第二种封禁用户的方式是在“评论审核”页面的每条评论上点击封禁按钮。

当我们点击封禁按钮时，会出现一些选项，我们可以在其中指定封禁类型和持续时间。

### 电子邮件别名

通过电子邮件封禁用户时，FastComments 会自动忽略 `+` 别名。例如，封禁 `user+alias@gmail.com` 也会封禁 `user@gmail.com` 以及该地址的任何其他 `+` 变体，例如 `user+other@gmail.com`。

### 影子封禁

影子封禁是一种封禁方式，它会让用户觉得他们的评论或投票已成功保存，实际上并未保存。在某些情况下，这可能是理想的做法。

### 通过 IP 地址封禁

除非租户选择退出，否则 FastComments 支持通过 IP 进行封禁，方法是存储评论者 IP 地址的哈希版本。

---