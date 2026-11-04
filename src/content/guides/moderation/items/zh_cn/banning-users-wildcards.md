可以使用通配符来封禁使用某些电子邮件服务商的用户。

例如，如果您发现来自 **@bademail.com** 的所有评论都是垃圾信息，您只需在添加被封禁用户时，在电子邮件输入框中输入 "*@bademail.com"，即可封禁整个电子邮件服务商。

请注意电子邮件中 @ 之前的 "*"。

### 子域名

封禁某个域名也会覆盖该域名的所有子域名。封禁 `*@bademail.com` 同时也会封禁
`someone@mail.bademail.com` 和 `someone@eu.mail.bademail.com`，因此无需为每个子域名单独添加封禁。

如果您只想封禁某个特定的子域名，请改为输入该子域名，例如 `*@mail.bademail.com`。该封禁
不会影响 `someone@bademail.com`。

### 从评论中封禁域名

您不必自己手动输入模式。当您在“审核评论”页面上从某条评论中封禁用户时，封禁对话框中有一个
"Ban All @domain Users"（封禁所有 @domain 用户）复选框，它会为评论者的电子邮件域名创建相同的 `*@domain` 封禁。

### 支持的模式

唯一支持的通配符形式是用单个 `*` 代替整个名称部分，后跟 `@` 和域名。其他形式
在尝试保存时会被拒绝：

- `*@*.bademail.com` 是不必要的，因为 `*@bademail.com` 已经覆盖了子域名。
- `name*@bademail.com` 和 `*bademail.com` 不受支持。