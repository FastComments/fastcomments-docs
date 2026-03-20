---
有两种方法可以使用 FastComments 禁止用户在您的网站上发表评论。

第一种是如果您已经知道他们的电子邮件，可以在 <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">banned users</a> 页面输入。

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

此页面可通过 Moderate Comments -> Banned Users 访问

当我们要封禁用户时，可以选择一种类型：永久封禁或永久影子封禁：

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

第二种封禁用户的方法是在评论审核页面上点击放置在每条评论上的封禁按钮。

当我们点击封禁按钮时，会出现一些选项，我们可以在其中指定封禁类型和持续时间。

### 影子封禁

影子封禁是一种封禁类型，它会让看起来用户的评论或投票已成功保存，但实际上并未保存。在某些情况下，这可能是可取的。

### 通过 IP 地址封禁

除非租户选择退出，否则 FastComments 支持通过 IP 封禁，方法是存储评论者 IP 地址的哈希版本。

---