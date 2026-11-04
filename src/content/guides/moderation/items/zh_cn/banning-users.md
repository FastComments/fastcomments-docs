有两种方式可以使用 FastComments 禁止用户在您的网站上评论。

第一种是如果您已经知道他们的电子邮箱，可以在<a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">被禁用户</a>页面中输入该邮箱。

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='“审核评论”下的被禁用户列表，包含被禁的电子邮箱地址和一个添加新禁令的按钮'; title='被禁用户页面' app-screenshot-end]

可以通过 审核评论 -> 被禁用户 访问此页面。

当我们去封禁一个用户时，可以选择一种类型，即永久封禁或永久影子封禁：

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='新禁令表单，包含电子邮箱字段以及“永久封禁”或“永久影子封禁”的封禁类型选择'; title='封禁用户' app-screenshot-end]

第二种封禁用户的方式是点击评论审核页面上每条评论旁边的封禁按钮。

当我们点击封禁按钮时，会看到一些选项，可以在其中指定封禁类型和时长。

### 电子邮箱别名

通过电子邮箱封禁用户时，FastComments 会自动忽略 `+` 别名。例如，封禁 `user+alias@gmail.com` 也会封禁 `user@gmail.com` 以及该地址的任何其他 `+` 变体，例如 `user+other@gmail.com`。

### 影子封禁

影子封禁是一种让用户看起来其评论或投票已成功保存的封禁方式，但实际上并未保存。在某些情况下这可能是理想的选择。

### 通过 IP 地址封禁

除非租户选择退出，FastComments 支持通过存储评论者 IP 地址的哈希版本来实现基于 IP 的封禁。

### 搜索被禁用户

当您的列表增长超过一两页时，可以使用表格上方的搜索行来缩小范围。

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='被禁用户页面上的搜索行，包含“搜索字段”下拉框、“匹配方式”下拉框和“值”输入框'; title='搜索被禁用户' app-screenshot-end]

共有三个控件：

- **搜索字段** 选择要在哪个字段中查找：任意字段、电子邮箱、姓名、封禁者或因言论被封。后四项对应表格中同名的列。
- **匹配方式** 选择如何比较。**包含** 会在字段中的任意位置查找您的值，**等于** 则匹配整个字段。
- **值** 是要查找的文本。

所有字段的匹配都不区分大小写，因此搜索 `SPAMMER@EXAMPLE.COM` 可以找到存储为 `spammer@example.com` 的封禁记录。

一些值得了解的事项：

- **因言论被封** 会搜索导致用户被封的评论文本。通过这种方式，您可以找到所有因某个特定短语而被封的用户。
- **封禁者** 会搜索执行封禁的审核员的姓名，这对于审查其他审核员的决定很有用。
- 通配符封禁会连同其 `*` 一起存储，因此对 `bademail.com` 进行**包含**搜索可以找到 `*@bademail.com` 的封禁记录。
- **姓名** 匹配“姓名”列中显示的姓名，因此即使用户在被封后更改了姓名，甚至当您通过输入电子邮箱创建封禁且当时未记录姓名时，也能找到该用户。封禁记录中记录的姓名仍然会匹配，因此搜索旧姓名或当前姓名都可以。
- **任意字段** 会同时搜索电子邮箱、姓名、执行封禁的审核员以及被封的评论文本。

您的搜索条件是页面 URL 的一部分，因此您可以像分享其他审核链接一样，与其他审核员分享筛选后的列表。翻页浏览结果时会保持搜索条件，开始新的搜索会返回到第一页，而**清除**则会返回完整列表。