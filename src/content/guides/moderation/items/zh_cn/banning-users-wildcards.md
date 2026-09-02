---
可以使用通配符禁止使用特定电子邮件提供商的用户。

例如，如果您发现来自 **@bademail.com** 的所有评论都是垃圾评论，您可以通过在添加被禁止用户时的电子邮件输入字段中输入 "*@bademail.com" 来直接禁止整个电子邮件提供商。

请注意电子邮件中 @ 前面的 "*"。

### Subdomains

域名封禁同样适用于该域名的所有子域。封禁 `*@bademail.com` 也会封禁 `someone@mail.bademail.com` 和 `someone@eu.mail.bademail.com`，因此无需为每个子域单独添加封禁。

如果您只想封禁特定子域，请改为输入该子域，例如 `*@mail.bademail.com`。此封禁不会影响 `someone@bademail.com`。

### Banning a Domain From a Comment

您无需手动输入模式。当您在“审核评论”页面从评论中封禁用户时，封禁对话框中有一个 "Ban All @domain Users" 复选框，可为评论者的电子邮件域创建相同的 `*@domain` 封禁。

### Supported Patterns

唯一受支持的通配符形式是用单个 `*` 替代完整的用户名部分，后跟 `@` 和域名。其他形式在保存时会被拒绝：

- `*@*.bademail.com` 并不需要，因为 `*@bademail.com` 已经覆盖了子域。
- `name*@bademail.com` 和 `*bademail.com` 不受支持。

---