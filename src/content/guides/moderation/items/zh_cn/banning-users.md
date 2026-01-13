使用 FastComments，有两种方法可以禁止用户在您的网站发表评论。

第一种是，如果您已经知道他们的电子邮件，您可以在 <a href="/auth/my-account/moderate-comments/banned-users" target="_blank">封禁用户</a> 页面中输入该电子邮件。

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

此页面可通过 评论管理 -> 封禁用户 访问

当我们要封禁用户时，可以选择一种类型，要么是永久封禁，要么是永久影子封禁：

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

第二种方法是在评论管理页面的每条评论上点击放置的封禁按钮来封禁用户。

当我们点击封禁按钮时，会显示一些选项，在那里我们可以指定封禁类型和持续时间。

### 影子封禁

影子封禁是一种封禁方式，它会让用户感觉他们的评论或投票已成功保存，而事实上并没有保存。
在某些情况下，这可能是
可取的。

### 通过 IP 地址封禁

除非租户选择退出，否则 FastComments 支持通过存储评论者 IP 地址的哈希版本来进行 IP 封禁。