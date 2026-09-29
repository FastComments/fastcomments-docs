---
当用户发表评论或投票且未登录时，系统会要求他们提供电子邮件和用户名。

对于某些站点来说，要求访客自行想出唯一的用户名是一道障碍，尤其在移动端。FastComments 可以为每位新访客生成一个中性用户名，并预填到用户名字段中，例如 `BraveOtter4172`。

访客可以保持原样，或替换为自己选择的名称。

可以在自定义 UI 中的 `Generate Usernames Automatically` 设置下启用此功能：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='小部件自定义 UI 中的“自动生成用户名”选项'; title='自动生成用户名' app-screenshot-end]

#### 工作方式

- 每个生成的名称都是唯一的。系统会检查现有账户并为该访客的浏览器会话保留该名称，确保不会向两个访客提供相同的名称。
- 仅为尚未拥有用户名的访客生成名称。已登录用户、SSO 用户以及已发表评论的访客会保留其现有名称。
- 它在开启或关闭[匿名评论](/guide-customizations-and-configuration.html#allow-anon)时均可工作。关闭匿名评论后，访客仍需输入电子邮件，但不再需要想出用户名。
- 再次访问的访客如果输入之前使用过的电子邮件，系统会将其匹配到已有账户，并保留该账户的用户名。
- 如果同时设置了[默认用户名](/guide-customizations-and-configuration.html#default-username)，生成的名称将优先使用。

---