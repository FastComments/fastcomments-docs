---
当用户发表评论或投票且未登录时，系统会要求他们提供电子邮件和用户名。

在匿名评论的情况下，有时希望定义一个默认用户名以降低评论时的阻力。这可以在自定义 UI 中完成。必须先启用匿名评论。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allow-anonymous-comments']; selector = '.default-username-label'; alt='在启用匿名评论后出现在自定义 UI 中的默认用户名字段'; title='设置默认用户名' app-screenshot-end]

#### 共享默认用户名

默认用户名是共享的显示名称，而非身份标识。保留默认用户名并输入电子邮件的访客会获得自己的账户，默认用户名会显示为他们的公开名称。任意数量的访客都可以使用相同的默认用户名，因此“Anonymous”永远不会被报告为已被占用。

访客自行输入的用户名仍需唯一。

如果您希望每位访客获得一个独特的名称而无需自行思考，请参阅[自动生成用户名](/guide-customizations-and-configuration.html#auto-generate-username)。